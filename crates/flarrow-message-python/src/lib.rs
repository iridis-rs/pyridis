use arrow::{array::ArrayData, pyarrow::PyArrowType};
use pyo3::{prelude::*, types::PyType};

#[pyclass(subclass)]
struct ArrowMessage {}

#[pymethods]
impl ArrowMessage {
    #[classmethod]
    fn from_arrow(cls: &Bound<'_, PyType>, array: PyArrowType<ArrayData>) -> PyResult<Self> {
        println!("{:?}", cls.getattr("__dict__"));
        println!("{:?}", cls.getattr("__dataclass_fields__"));

        Ok(ArrowMessage {})
    }
}

#[pymodule]
fn pyflarrow_message(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ArrowMessage>()?;

    Ok(())
}
