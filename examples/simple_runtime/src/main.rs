use iridis::prelude::{thirdparty::*, *};

use pyridis_file_ext::prelude::PythonFileExtPlugin;

#[tokio::main]
async fn main() -> Result<()> {
    let mut layout = DataflowLayout::new();

    let (source, output) = layout
        .node("source", async |builder: &mut NodeIOBuilder| {
            builder.output("out")
        })
        .await;

    let (sink, input) = layout
        .node("sink", async |builder: &mut NodeIOBuilder| {
            builder.input("in")
        })
        .await;

    let layout = layout.build();

    let flows = Flows::new(layout.clone(), async move |builder: &mut FlowsBuilder| {
        builder.connect(input, output, None)?;

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
        .run(flows, async move |loader: &mut NodeLoader| {
            loader
                .load::<Timer>(source, serde_yml::from_str("frequency: 1.0")?)
                .await?;

            loader
                .load_url(Url::parse("file:///home/enzo/Documents/iridis/iridis-python/crates/pyridis-api/examples/sink.py")?, sink, serde_yml::from_str("")?)
                .await?;
            Ok(())
        })
        .await
}
