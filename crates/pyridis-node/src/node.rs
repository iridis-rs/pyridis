use std::ffi::CString;

use crate::prelude::{
    thirdparty::{
        ird::thirdparty::*,
        pyo3::{ffi::c_str, prelude::*, types::*},
    },
    *,
};

#[derive(ird::Node)]
pub struct PythonNode {
    pub instance: PyObject,
}

impl ird::Node for PythonNode {
    fn new(
        inputs: ird::Inputs,
        outputs: ird::Outputs,
        queries: ird::Queries,
        queryables: ird::Queryables,
        configuration: serde_yml::Value,
    ) -> tokio::task::JoinHandle<Result<Box<dyn ird::Node>>> {
        pyo3::prepare_freethreaded_python();

        let mut builder = tokio::runtime::Builder::new_current_thread();
        builder.enable_all();

        pyo3_async_runtimes::tokio::init(builder);

        std::thread::spawn(|| {
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(pyo3_async_runtimes::tokio::re_exports::pending::<()>())
        });

        pyo3_async_runtimes::tokio::get_runtime().spawn_blocking(move || {
            Python::with_gil(|py| {
                pyo3_async_runtimes::tokio::run(py, async move {
                    let file_path = configuration
                        .get("python_file_path")
                        .ok_or_eyre("Cannot find python file path inside configuration")?
                        .as_str();

                    let file_path = file_path
                        .ok_or_eyre(format!("Invalid python file path: '{:?}'", file_path))?;

                    let py_script = tokio::fs::read_to_string(file_path)
                        .await
                        .wrap_err(format!("Couldn't read path '{}'", file_path))?;

                    let module: PyObject = Python::with_gil(|py| -> Result<PyObject> {
                        Ok(PyModule::from_code(
                            py,
                            CString::new(py_script)?.as_c_str(),
                            CString::new(file_path)?.as_c_str(),
                            c_str!("pyridis_node"),
                        )?
                        .into())
                    })?;

                    let instance: PyObject = Python::with_gil(|py| -> PyResult<PyObject> {
                        let class = module.call_method0(py, "pyridis_node")?;

                        class.call0(py)
                    })?;

                    let configuration: PyObject = Python::with_gil(|py| -> PyResult<PyObject> {
                        // TODO convert 'configuration' to a Python dictionary
                        Ok(PyDict::new(py).into())
                    })?;

                    let inputs = Inputs(inputs);
                    let outputs = Outputs(outputs);
                    let queries = Queries(queries);
                    let queryables = Queryables(queryables);

                    let fut = Python::with_gil(|py| {
                        pyo3_async_runtimes::tokio::into_future(
                            instance
                                .call_method1(
                                    py,
                                    "new",
                                    (inputs, outputs, queries, queryables, configuration),
                                )?
                                .into_bound(py),
                        )
                    })?;

                    fut.await
                        .wrap_err("Couldn't await for the instance call to 'new'")?;

                    Ok(Box::new(Self { instance }) as Box<dyn ird::Node>)
                })
                .map_err(|e| {
                    e.print_and_set_sys_last_vars(py);

                    eyre::eyre!(e)
                })
            })
        })
    }

    fn start(self: Box<Self>) -> tokio::task::JoinHandle<Result<()>> {
        pyo3_async_runtimes::tokio::get_runtime().spawn_blocking(move || {
            Python::with_gil(|py| {
                pyo3_async_runtimes::tokio::run(py, async move {
                    {
                        let fut = Python::with_gil(|py| {
                            pyo3_async_runtimes::tokio::into_future(
                                self.instance.call_method0(py, "start")?.into_bound(py),
                            )
                        })?;

                        fut.await
                            .wrap_err("Couldn't await for the instance call to 'start'")?;

                        Ok(())
                    }
                })
                .map_err(|e| {
                    e.print_and_set_sys_last_vars(py);

                    eyre::eyre!(e)
                })
            })
        })
    }
}
