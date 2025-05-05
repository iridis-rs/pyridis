use iridis::prelude::{thirdparty::*, *};

use pyridis_file_ext::PythonFileExtPlugin;

#[tokio::main]
async fn main() -> Result<()> {
    let mut layout = DataflowLayout::new();

    let (sink, input) = layout
        .node("sink", async |builder: &mut NodeIOBuilder| {
            builder.input("in")
        })
        .await;

    let layout = layout.build();

    let flows = Flows::new(layout.clone(), async move |builder: &mut FlowsBuilder| {
        Ok(())
    })
    .await?;

    let runtime = Runtime::new(
        async |file_ext: &mut FileExtManagerBuilder, _url_scheme: &mut UrlSchemeManagerBuilder| {
            file_ext
                .load_statically_linked_plugin::<PythonFileExtPlugin>()
                .await?;

            Ok(())
        },
    )
    .await?;

    runtime
        .run(flows, async move |_loader: &mut NodeLoader| Ok(()))
        .await
}
