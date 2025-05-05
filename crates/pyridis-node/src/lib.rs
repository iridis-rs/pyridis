use std::ffi::CString;

use ird::Node;

use iridis_api::prelude::{self as ird, thirdparty::*};
use pyo3::{
    PyObject, PyResult, Python,
    ffi::c_str,
    types::{PyDict, PyModule},
};
use pyridis_api::*;

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
    ) -> tokio::task::JoinHandle<Result<Box<dyn Node>>> {
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
                        .as_str()
                        .ok_or_eyre("Invalid python file path")?;

                    let py_script = tokio::fs::read_to_string(file_path).await?;

                    let module: PyObject = Python::with_gil(|py| -> PyResult<PyObject> {
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

                        Ok(class.call0(py)?.into())
                    })?;

                    let configuration: PyObject = Python::with_gil(|py| -> PyResult<PyObject> {
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

                    fut.await?;

                    Ok(Box::new(Self { instance }) as Box<dyn Node>)
                })
                .map_err(|e| {
                    e.print_and_set_sys_last_vars(py);

                    eyre::eyre!("")
                })
            })
        })

        // let module: PyObject = Python::with_gil(|py| -> PyResult<PyObject> {
        //     Ok(PyModule::from_code(
        //         py,
        //         CString::new(py_script)?.as_c_str(),
        //         CString::new(file_path)?.as_c_str(),
        //         c_str!("pyridis_node"),
        //     )?
        //     .into())
        // })?;

        // // let task_locals = Python::with_gil(|py| {
        // //     let asyncio: PyObject = py.import("asyncio")?.into();
        // //     let event_loop = asyncio.call_method0(py, "new_event_loop")?.into_bound(py);
        // //     asyncio.call_method1(py, "set_event_loop", (&event_loop,))?;

        // //     let task_locals = pyo3_async_runtimes::TaskLocals::new(event_loop);

        // //     Ok::<_, PyErr>(task_locals)
        // // })?;

        // // println!("Event loop set");

        // let instance: PyObject = Python::with_gil(|py| -> PyResult<PyObject> {
        //     let class = module.call_method0(py, "pyridis_node")?;

        //     Ok(class.call0(py)?.into())
        // })?;

        // println!("Instance created");

        // let future = Python::with_gil(|py| {
        //     let inputs = Inputs(inputs);
        //     let outputs = Outputs(outputs);
        //     let queries = Queries(queries);
        //     let queryables = Queryables(queryables);
        //     let configuration = PyDict::new(py);

        //     Ok::<_, PyErr>(pyo3_async_runtimes::tokio::into_future(
        //         instance
        //             .call_method1(
        //                 py,
        //                 "new",
        //                 (inputs, outputs, queries, queryables, configuration),
        //             )?
        //             .into_bound(py),
        //     )?)
        // })?;

        // println!("Future created");

        // future.await?;

        // println!("Future completed");

        // tokio::task::spawn(async move { Ok(Box::new(Self {}) as Box<dyn Node>) })
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

                        fut.await?;

                        Ok(())
                    }
                })
                .map_err(|e| {
                    e.print_and_set_sys_last_vars(py);

                    eyre::eyre!("")
                })
            })
        })
        // let future = Python::with_gil(|py| {
        //     Ok::<_, PyErr>(pyo3_async_runtimes::tokio::into_future(
        //         self.instance.call_method0(py, "start")?.into_bound(py),
        //     )?)
        // })?;

        // future.await?;

        // tokio::task:§:spawn(async move { Ok(()) })
    }
}
