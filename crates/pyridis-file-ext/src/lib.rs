use iridis_file_ext::prelude::{thirdparty::*, *};

use pyridis_node::PythonNode;

#[derive(FileExtPlugin)]
pub struct PythonFileExtPlugin {}

#[file_ext_plugin(runtime = "default_runtime")]
impl FileExtPlugin for PythonFileExtPlugin {
    async fn new() -> Result<Self>
    where
        Self: Sized,
    {
        Ok(PythonFileExtPlugin {})
    }

    fn target(&self) -> Vec<String> {
        vec!["py".to_string()]
    }

    async fn load(
        &self,
        path: std::path::PathBuf,

        inputs: Inputs,
        outputs: Outputs,
        queries: Queries,
        queryables: Queryables,
        configuration: serde_yml::Value,
    ) -> Result<iridis_runtime_core::prelude::RuntimeNode> {
        match path.extension() {
            Some(ext) => {
                if ext == "py" {
                    let mut configuration = configuration.clone();
                    configuration["python_file_path"] =
                        serde_yml::Value::String(path.to_string_lossy().to_string());

                    Ok(RuntimeNode::StaticallyLinked(
                        PythonNode::new(inputs, outputs, queries, queryables, configuration)
                            .await??,
                    ))
                } else {
                    Err(eyre::eyre!(
                        "Unsupported file extension '{:?}'. On this platform it must be '{}'",
                        ext,
                        std::env::consts::DLL_EXTENSION
                    ))
                }
            }
            None => Err(eyre::eyre!("No file extension found for path {:?}", path)),
        }
    }
}
