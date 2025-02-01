use color_eyre::eyre::{eyre, OptionExt, Result};
use ego_tree::NodeRef;
use scraper::{CaseSensitivity, ElementRef, Node, Selector};

#[derive(Debug)]
pub enum Word {
    Text(String),
    Kanji { word: String, furigana: String },
}

impl Word {
    pub fn try_from_noderef(node: NodeRef<'_, Node>) -> Option<Word> {
        if let Some(text) = node.value().as_text() {
            return Some(Word::Text(text.to_string()));
        }

        if let Some(ele) = ElementRef::wrap(node) {
            if ele
                .value()
                .has_class("ruby", CaseSensitivity::AsciiCaseInsensitive)
            {
                let word = ele
                    .select(&Selector::parse(".rb").unwrap())
                    .next()
                    .unwrap()
                    .inner_html();

                let furigana = ele
                    .select(&Selector::parse(".rt").unwrap())
                    .next()
                    .unwrap()
                    .inner_html();

                return Some(Word::Kanji { word, furigana });
            }
        }
        None
    }

    pub fn anki_format(&self) -> String {
        match self {
            Word::Text(text) => text.clone(),
            Word::Kanji { word, furigana } => format!(" {}[{}]", word, furigana),
        }
    }
}

/// Parse lyric from a lyric page
pub(crate) fn parse_lyric(document: &scraper::Html) -> Result<impl Iterator<Item = Word> + '_> {
    let lyrics = document
        .select(&Selector::parse(".hiragana").unwrap())
        .next()
        .ok_or_eyre("No hiragana lyrics found")?;

    Ok(lyrics
        .children()
        .filter_map(|node| Word::try_from_noderef(node)))
}

/// Extract lyrics URL from a search result page
pub(crate) fn extract_lyrics(document: &scraper::Html) -> Result<Vec<String>> {
    let selector = Selector::parse(".searchResult__title a").unwrap();
    let songs = document.select(&selector);

    songs
        .map(|song| {
            let href = song
                .value()
                .attr("href")
                .ok_or_else(|| eyre!("Song link has no href"))?;

            if href.starts_with("http") {
                Ok(href.to_string())
            } else {
                Ok(format!("https://utaten.com{}", href))
            }
        })
        .collect()
}
