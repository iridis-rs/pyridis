use pyridis_examples::prelude::ird::{thirdparty::*, *};

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
                .load_dynamically_linked_plugin(pyridis_examples::dylib("pyridis_file_ext", None)?)
                .await?;

            Ok(())
        },
    )
    .await?;

    runtime
        .run(flows, async move |loader: &mut NodeLoader| {
            loader
                .load_url(
                    pyridis_examples::pyfile("source.py")?,
                    source,
                    serde_yml::from_str("")?,
                )
                .await?;

            loader
                .load_url(
                    pyridis_examples::pyfile("sink.py")?,
                    sink,
                    serde_yml::from_str("")?,
                )
                .await?;
            Ok(())
        })
        .await
}
