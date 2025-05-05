use std::ffi::CString;

use ird::Node;

use iridis_api::prelude::{self as ird, thirdparty::*};
use pyo3::{
    PyErr, PyObject, PyResult, Python,
    ffi::c_str,
    types::{PyDict, PyModule},
};
use pyridis_api::*;

#[derive(ird::Node)]
pub struct PythonNode {
    pub obj: PyObject,
}

#[ird::node(runtime = "default_runtime")]
impl ird::Node for PythonNode {
    async fn new(
        inputs: ird::Inputs,
        outputs: ird::Outputs,
        queries: ird::Queries,
        queryables: ird::Queryables,
        configuration: serde_yml::Value,
    ) -> Result<Self> {
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

        let future = Python::with_gil(|py| {
            let inputs = Inputs(inputs);
            let outputs = Outputs(outputs);
            let queries = Queries(queries);
            let queryables = Queryables(queryables);
            let configuration = PyDict::new(py);

            Ok::<_, PyErr>(pyo3_async_runtimes::tokio::into_future(
                instance
                    .call_method1(
                        py,
                        "new",
                        (inputs, outputs, queries, queryables, configuration),
                    )?
                    .into_bound(py),
            )?)
        })?;

        future.await?;

        Ok(Self { obj: instance })
    }

    async fn start(self: Box<Self>) -> Result<()> {
        let future = Python::with_gil(|py| {
            Ok::<_, PyErr>(pyo3_async_runtimes::tokio::into_future(
                self.obj.call_method0(py, "start")?.into_bound(py),
            )?)
        })?;

        future.await?;

        Ok(())
    }
}
