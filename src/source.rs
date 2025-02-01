use std::{fs, path::Path};

use color_eyre::eyre::{eyre, Result};

use crate::parse;

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
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
        SourceType::Keyword => read_source_as_keyword(source).await,
    }
}

#[maybe_async::maybe_async]
async fn read_source_as_url(url: &str) -> Result<String> {
    get(url, &[]).await
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

#[maybe_async::maybe_async]
async fn read_source_as_keyword(keyword: &str) -> Result<String> {
    use scraper::Html;

    let url = "https://utaten.com/search";
    let query = &[
        ("sort", "popular_sort_asc"),
        ("artist_name", ""),
        ("title", keyword),
        ("beginning", ""),
        ("body", ""),
        ("lyricist", ""),
        ("composer", ""),
        ("sub_title", ""),
        ("tag", ""),
    ];

    let html = get(&url, query).await?;

    let urls = parse::extract_lyrics(&Html::parse_document(&html))?;

    let url = urls
        .first()
        .ok_or_else(|| eyre!("No lyric found for keyword: {}", keyword))?;
    read_source_as_url(url).await
}

#[maybe_async::sync_impl]
fn get(url: &str, query: &[(&str, &str)]) -> Result<String> {
    let bytes = ureq::get(url)
        .query_pairs(query.into_iter().copied())
        .call()?
        .into_body()
        .read_to_vec()?;
    let response = String::from_utf8_lossy(&bytes).into_owned();
    Ok(response)
}

#[maybe_async::async_impl]
async fn get(url: &str, query: &[(&str, &str)]) -> Result<String> {
    let response = reqwest::Client::new()
        .get(url)
        .query(query)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;
    Ok(response)
}
