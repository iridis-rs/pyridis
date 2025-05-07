use iridis_api::prelude::DataflowMessage;

use crate::prelude::{
    thirdparty::{
        arrow::{array::*, pyarrow::*},
        pyo3::prelude::*,
    },
    *,
};

#[pyclass]
pub struct Inputs(pub ird::Inputs);

#[pymethods]
impl Inputs {
    pub async fn with_input(&mut self, input: String) -> PyResult<Input> {
        let input = self.0.raw(input).await?;

        Ok(Input(input))
    }
}

#[pyclass]
pub struct Outputs(pub ird::Outputs);

#[pymethods]
impl Outputs {
    pub async fn with_output(&mut self, output: String) -> PyResult<Output> {
        let output = self.0.raw(output).await?;

        Ok(Output(output))
    }
}

#[pyclass]
pub struct Queries(pub ird::Queries);

#[pymethods]
impl Queries {
    pub async fn with_query(&mut self, query: String) -> PyResult<Query> {
        let query = self.0.raw(query).await?;

        Ok(Query(query))
    }
}

#[pyclass]
pub struct Queryables(pub ird::Queryables);

#[pymethods]
impl Queryables {
    pub async fn with_queryable(&mut self, queryable: String) -> PyResult<Queryable> {
        let queryable = self.0.raw(queryable).await?;

        Ok(Queryable(queryable))
    }
}

#[pyclass]
pub struct Header(pub ird::Header);

#[pymethods]
impl Header {
    #[getter]
    pub fn source_node(&self) -> String {
        let (a, _) = self.0.source;
        a.to_string()
    }

    #[getter]
    pub fn source_io(&self) -> String {
        let (_, b) = self.0.source;
        b.to_string()
    }

    #[getter]
    pub fn elapsed(&self) -> u128 {
        let elapsed = self
            .0
            .timestamp
            .get_time()
            .to_system_time()
            .elapsed()
            .unwrap_or_default()
            .as_nanos();

        elapsed
    }
}

#[pyclass]
pub struct PyDataflowMessage {
    pub data: PyArrowType<ArrayData>,
    pub header: Header,
}

#[pymethods]
impl PyDataflowMessage {
    #[getter]
    pub fn data(&self) -> PyArrowType<ArrayData> {
        let array = self.data.0.clone();

        PyArrowType(array)
    }

    #[getter]
    pub fn header(&self) -> Header {
        let header = self.header.0.clone();

        Header(header)
    }
}

#[pyclass]
pub struct Input(pub ird::RawInput);

#[pymethods]
impl Input {
    pub async fn recv(&mut self) -> PyResult<PyDataflowMessage> {
        let DataflowMessage { header, data } = self.0.recv().await?;

        Ok(PyDataflowMessage {
            data: PyArrowType(data),
            header: Header(header),
        })
    }
}

#[pyclass]
pub struct Output(pub ird::RawOutput);

#[pymethods]
impl Output {
    pub async fn send(&mut self, data: PyArrowType<ArrayData>) -> PyResult<()> {
        self.0.send(data.0).await?;

        Ok(())
    }
}

#[pyclass]
pub struct Query(pub ird::RawQuery);

#[pymethods]
impl Query {
    pub async fn query(&mut self, data: PyArrowType<ArrayData>) -> PyResult<PyDataflowMessage> {
        let DataflowMessage { header, data } = self.0.query(data.0).await?;

        Ok(PyDataflowMessage {
            data: PyArrowType(data),
            header: Header(header),
        })
    }
}

#[pyclass]
pub struct Queryable(pub ird::RawQueryable);

// TODO: should accept async python callbacks
#[pymethods]
impl Queryable {
    pub async fn on_query(&mut self, response: PyObject) -> PyResult<()> {
        self.0
            .on_query(async |query: DataflowMessage| {
                let DataflowMessage { header, data } = query;
                let message = PyDataflowMessage {
                    data: PyArrowType(data),
                    header: Header(header),
                };

                let array = Python::with_gil(|py| -> PyResult<ArrayData> {
                    let array = response.call1(py, (message,))?.into_bound(py);

                    ArrayData::from_pyarrow_bound(&array)
                })?;

                Ok(array)
            })
            .await?;

        Ok(())
    }
}
