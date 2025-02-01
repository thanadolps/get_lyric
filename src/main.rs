use clap::Parser;
use color_eyre::eyre::Result;

#[derive(Parser, Debug)]
struct Args {
    /// Source of the lyrics, either a keyword, a html file, or a url
    source: String,

    /// Force the source type instead of auto-detecting from <SOURCE>
    #[clap(short, long)]
    source_type: Option<get_lyric::SourceType>,

    /// Output the result as raw html
    #[clap(short, long)]
    output_html: bool,
}

#[maybe_async::async_impl]
#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    run(Args::parse()).await
}

#[maybe_async::sync_impl]
fn main() -> Result<()> {
    color_eyre::install()?;
    run(Args::parse())
}

#[maybe_async::maybe_async]
async fn run(args: Args) -> Result<()> {
    if args.output_html {
        let html = get_lyric::get_html(&args.source, args.source_type).await?;
        println!("{}", html);
        return Ok(());
    }

    let output = get_lyric::get(&args.source, args.source_type).await?;
    println!("{}", output.trim());
    Ok(())
}
