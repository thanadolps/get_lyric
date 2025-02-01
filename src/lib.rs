mod parse;
mod source;

pub use source::SourceType;

use color_eyre::eyre::{Context, Result};

#[maybe_async::maybe_async]
pub async fn get(source: &str, source_type: Option<SourceType>) -> Result<String> {
    let html = get_html(source, source_type).await?;
    let lyrics = from_html(&html)?;
    Ok(lyrics)
}

#[maybe_async::maybe_async]
pub async fn get_html(source: &str, source_type: Option<SourceType>) -> Result<String> {
    let html = if let Some(source_type) = source_type {
        source::read_source_as(&source, source_type).await
    } else {
        source::read_source(&source).await
    }
    .wrap_err_with(|| format!("cannot read from source `{}`", source))?;

    Ok(html)
}

pub fn from_html(html: &str) -> Result<String> {
    let mut output = parse::parse(&scraper::Html::parse_document(&html))?
        .map(|word| word.anki_format())
        .collect::<String>();

    // trim space (not newline) from each line
    output = output
        .lines()
        .map(|line| line.trim_matches(' '))
        .collect::<Vec<_>>()
        .join("\n");

    Ok(output.trim().to_string())
}
