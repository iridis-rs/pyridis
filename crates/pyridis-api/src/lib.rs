use iridis_api::prelude as ird;

use pyo3::prelude::*;

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
pub struct Input(pub ird::RawInput);

#[pyclass]
pub struct Output(pub ird::RawOutput);

#[pyclass]
pub struct Query(pub ird::RawQuery);

#[pyclass]
pub struct Queryable(pub ird::RawQueryable);

#[pymodule]
fn pyridis_api(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Inputs>()?;
    m.add_class::<Outputs>()?;
    m.add_class::<Queries>()?;
    m.add_class::<Queryables>()?;

    m.add_class::<Input>()?;
    m.add_class::<Output>()?;
    m.add_class::<Query>()?;
    m.add_class::<Queryable>()?;

    Ok(())
}
