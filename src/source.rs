use std::{fs, path::Path};

use color_eyre::{
    eyre::{eyre, Result},
    Section,
};

#[derive(Debug, Copy, Clone, clap::ValueEnum)]
pub enum SourceType {
    Html,
    Url,
    Keyword,
}

#[maybe_async::maybe_async]
pub async fn read_source(source: &str) -> Result<String> {
    let source = source.trim();
    let source_type = determine_source_type(source);
    read_source_as(source, source_type).await
}

#[maybe_async::maybe_async]
pub async fn read_source_as(source: &str, source_type: SourceType) -> Result<String> {
    let source = source.trim();
    match source_type {
        SourceType::Html => Ok(fs::read_to_string(source)?),
        SourceType::Url => read_source_as_url(source).await,
        SourceType::Keyword => Err(eyre!("keyword search not implemented yet"))
            .suggestion("try using a file or url instead"),
    }
}

#[maybe_async::async_impl]
async fn read_source_as_url(url: &str) -> Result<String> {
    Ok(reqwest::get(url).await?.error_for_status()?.text().await?)
}

#[maybe_async::sync_impl]
fn read_source_as_url(url: &str) -> Result<String> {
    Ok(ureq::get(url).call()?.into_body().read_to_string()?)
}

fn determine_source_type(source: &str) -> SourceType {
    let source = source.trim();

    if source.ends_with(".html") || Path::new(source).exists() {
        return SourceType::Html;
    }

    if source.starts_with("https://utaten.com/lyric") {
        return SourceType::Url;
    }

    SourceType::Keyword
}
