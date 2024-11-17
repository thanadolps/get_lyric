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

pub async fn read_source(source: &str) -> Result<String> {
    let source = source.trim();
    let source_type = determine_source_type(source);
    read_source_as(source, source_type).await
}

pub async fn read_source_as(source: &str, source_type: SourceType) -> Result<String> {
    let source = source.trim();
    match source_type {
        SourceType::Html => Ok(fs::read_to_string(source)?),
        SourceType::Url => Ok(reqwest::get(source)
            .await?
            .error_for_status()?
            .text()
            .await?),
        SourceType::Keyword => Err(eyre!("keyword search not implemented yet"))
            .suggestion("try using a file or url instead"),
    }
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
