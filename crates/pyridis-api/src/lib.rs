pub(crate) mod io;

pub mod prelude {
    pub use crate::io::*;

    pub(crate) use thirdparty::*;

    pub mod thirdparty {
        pub use arrow;
        pub use pyo3;
        pub use pyo3_async_runtimes;

        pub use iridis_api::prelude as ird;
    }
}

use prelude::*;

#[pyo3::pymodule]
fn pyridis_api(m: &pyo3::Bound<'_, pyo3::types::PyModule>) -> pyo3::PyResult<()> {
    use pyo3::types::*;

    m.add_class::<Inputs>()?;
    m.add_class::<Outputs>()?;
    m.add_class::<Queries>()?;
    m.add_class::<Queryables>()?;

    m.add_class::<Input>()?;
    m.add_class::<Output>()?;
    m.add_class::<Query>()?;
    m.add_class::<Queryable>()?;

    m.add_class::<Header>()?;
    m.add_class::<PyDataflowMessage>()?;

    Ok(())
}
