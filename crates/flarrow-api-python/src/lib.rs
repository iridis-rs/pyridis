use pyo3::prelude::*;

static DEFAULT_TOKIO_RUNTIME: std::sync::LazyLock<tokio::runtime::Runtime> =
    std::sync::LazyLock::new(|| {
        tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime")
    });

#[pyfunction]
async fn sleep(seconds: f64) -> PyResult<()> {
    DEFAULT_TOKIO_RUNTIME
        .spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis((seconds * 1000.0) as u64)).await;
        })
        .await
        .unwrap();

    Ok(())
}

#[pyo3::pyclass]
struct MyClass {
    value: i32,
}

#[pyo3::pymethods]
impl MyClass {
    #[new]
    fn new(value: i32) -> Self {
        MyClass { value }
    }

    fn get_value(&self) -> i32 {
        self.value
    }

    fn set_value(&mut self, value: i32) {
        self.value = value;
    }
}

#[pyo3::pyclass]
struct MyOtherClass(Option<MyClass>);

#[pyo3::pymethods]
impl MyOtherClass {
    #[new]
    fn new(value: i32) -> Self {
        MyOtherClass(Some(MyClass::new(value)))
    }

    fn set_value(&mut self, value: i32) {
        if let Some(ref mut my_class) = self.0 {
            my_class.set_value(value);
        }
    }

    async fn print_after(&mut self, seconds: f64) {
        let a = self.0.take().unwrap();
        let b = DEFAULT_TOKIO_RUNTIME
            .spawn(async move {
                tokio::time::sleep(std::time::Duration::from_millis((seconds * 1000.0) as u64))
                    .await;
                println!("Value: {}", a.get_value());
                a
            })
            .await
            .unwrap();

        self.0 = Some(b);
    }
}

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
fn flarrow_api_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(sleep, m)?)?;

    m.add_class::<MyClass>()?;
    m.add_class::<MyOtherClass>()?;

    Ok(())
}
