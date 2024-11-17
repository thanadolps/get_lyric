mod parse;
mod source;

use clap::Parser;
use color_eyre::eyre::{Context, Result};

#[derive(Parser, Debug)]
struct Args {
    /// Source of the lyrics, either a keyword, a html file, or a url
    source: String,

    /// Force the source type instead of auto-detecting from <SOURCE>
    #[clap(short, long)]
    source_type: Option<source::SourceType>,

    /// Output the result as raw html
    #[clap(short, long)]
    output_html: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    run(Args::parse()).await
}

async fn run(args: Args) -> Result<()> {
    let html = if let Some(source_type) = args.source_type {
        source::read_source_as(&args.source, source_type).await
    } else {
        source::read_source(&args.source).await
    }
    .wrap_err_with(|| format!("cannot read from source `{}`", args.source))?;

    if args.output_html {
        println!("{}", html);
        return Ok(());
    }

    let mut output = parse::parse(&scraper::Html::parse_document(&html))?
        .map(|word| word.anki_format())
        .collect::<String>();

    // trim space (not newline) from each line
    output = output
        .lines()
        .map(|line| line.trim_matches(' '))
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", output);

    Ok(())
}
