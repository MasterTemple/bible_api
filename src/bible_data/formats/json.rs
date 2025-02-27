use std::path::Path;

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::bible_data::bible_data::{
    AbbreviationsToBookId, BibleContents, BibleData, BookDataList, BookIdToName, ChapterDataList,
    ReferenceArray, VerseData, VerseDataList,
};

use super::parse::ParseBibleData;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JSONTranslation {
    pub name: String,
    pub language: String,
    pub abbreviation: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JSONBook {
    /// book id where Genesis = 1
    pub id: usize,
    /// the name of the book as it is displayed
    pub book: String,
    /// all abbreviations (any case), not necessarily including the book name
    pub abbreviations: Vec<String>,
    pub content: Vec<Vec<Option<String>>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JSONBible {
    pub translation: JSONTranslation,
    pub bible: Vec<JSONBook>,
}

impl ParseBibleData for JSONBible {
    fn parse_file(path: &Path) -> Result<JSONBible, Box<dyn std::error::Error>> {
        let contents = &std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(contents)?)
    }

    fn as_bible_data(self) -> Result<BibleData, Box<dyn std::error::Error>> {
        let mut abbreviations_to_book_id = AbbreviationsToBookId::new();
        let mut book_id_to_name = BookIdToName::new();
        let mut reference_array = ReferenceArray::new();
        let mut bible_contents: Vec<ChapterDataList> = Vec::new();

        // let translation = self.translation;

        for book in self.bible.into_iter() {
            // let mut book_contents: Vec<Vec<String>> = vec![];
            let mut book_contents: Vec<VerseDataList> = vec![];
            book_id_to_name.insert(book.id, book.book.clone());
            abbreviations_to_book_id.insert(book.book.clone().to_lowercase(), book.id);
            for abbreviation in book.abbreviations.iter().cloned() {
                abbreviations_to_book_id.insert(abbreviation.to_lowercase(), book.id);
            }
            let mut chapter_array = Vec::new();
            for (_, verses) in book.content.into_iter().enumerate() {
                chapter_array.push(verses.len());
                let verses = VerseDataList(
                    verses
                        .into_iter()
                        .map(|content| VerseData { content })
                        .collect::<Vec<_>>(),
                );
                book_contents.push(verses);
            }
            reference_array.push(chapter_array);
            bible_contents.push(ChapterDataList(book_contents));
        }

        let books_pattern: String = join("|", abbreviations_to_book_id.keys());
        // let books_pattern: String = abbreviations_to_book_id
        //     .keys()
        //     .map(|key| key.to_string())
        //     .collect::<Vec<String>>()
        //     .join("|");
        // I added the period so that people can use it in abbreviations
        let book_regex = Regex::new(format!(r"\b((?i){books_pattern})\b\.?").as_str())
            .expect("Failed to compile book_regex.");

        Ok(BibleData {
            book_regex,
            // translation,
            abbreviations_to_book_id,
            book_id_to_name,
            reference_array,
            bible_contents: BookDataList(bible_contents),
        })
    }
}

use std::fmt::Write;
/// https://users.rust-lang.org/t/connecting-joining-string-slices-without-a-temp-vec/1811/6
fn join<T>(sep: &str, iter: T) -> String
where
    T: IntoIterator,
    T::Item: std::fmt::Display,
{
    let mut out = String::new();
    let mut iter = iter.into_iter();
    if let Some(fst) = iter.next() {
        write!(&mut out, "{}", fst).unwrap();
        for elt in iter {
            write!(&mut out, "{}{}", sep, elt).unwrap();
        }
    }
    out
}
