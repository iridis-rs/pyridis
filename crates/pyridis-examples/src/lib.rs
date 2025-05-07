pub mod prelude {
    pub use thirdparty::*;

    pub mod thirdparty {
        pub use iridis::prelude as ird;
    }
}

use std::path::PathBuf;

use iridis::prelude::thirdparty::*;

pub fn dylib(name: &str, build: Option<&str>) -> Result<PathBuf> {
    let path = std::env::var("CARGO_MANIFEST_DIR")?;
    let path = format!("{}/../../target/{}", path, build.unwrap_or("debug"));

    let prefix = std::env::consts::DLL_PREFIX;
    let dylib = std::env::consts::DLL_SUFFIX;

    Ok(PathBuf::from(&format!(
        "{}/{}{}{}",
        path, prefix, name, dylib
    )))
}

pub fn pyfile(name: &str) -> Result<Url> {
    let path = std::env::var("CARGO_MANIFEST_DIR")?;
    let path = format!("file://{}/examples", path);

    Url::parse(&format!("{}/{}", path, name)).map_err(eyre::Report::msg)
}
