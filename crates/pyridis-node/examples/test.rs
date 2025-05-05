use std::{ffi::CString, time::Duration};

use pyo3::{ffi::c_str, prelude::*};

async fn my_main() -> PyResult<()> {
    let file_path =
        "/home/enzo/Documents/iridis/iridis-python/crates/pyridis-api/examples/example.py";

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

    let fut = Python::with_gil(|py| {
        pyo3_async_runtimes::tokio::into_future(instance.call_method0(py, "start")?.into_bound(py))
    })?;

    fut.await?;

    Ok(())
}

fn tokio_main() -> tokio::task::JoinHandle<eyre::Result<()>> {
    pyo3_async_runtimes::tokio::get_runtime().spawn_blocking(|| {
        Python::with_gil(|py| {
            pyo3_async_runtimes::tokio::run(py, async {
                my_main().await?;

                Ok(())
            })
            .map_err(|e| {
                e.print_and_set_sys_last_vars(py);
            })
            .unwrap();
        });

        Ok(())
    })
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    pyo3::prepare_freethreaded_python();

    let mut builder = tokio::runtime::Builder::new_current_thread();
    builder.enable_all();

    pyo3_async_runtimes::tokio::init(builder);

    std::thread::spawn(|| {
        pyo3_async_runtimes::tokio::get_runtime()
            .block_on(pyo3_async_runtimes::tokio::re_exports::pending::<()>())
    });

    let t1 = tokio::spawn(async { tokio_main().await? });
    let t2 = tokio::spawn(async {
        for _ in 0..10 {
            println!("AAA");
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    });

    t1.await??;
    t2.await?;

    Ok(())
}
