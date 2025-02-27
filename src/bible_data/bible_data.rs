use std::{
    collections::BTreeMap,
    ops::{Deref, RangeInclusive},
    path::Path,
};

use regex::Regex;

use crate::api::bible_api::ApiData;

use super::{book::BibleBook, chapter::BibleChapter, verse::BibleVerse};

/// map of abbreviations and actual name (all lowercase) to book id
pub type AbbreviationsToBookId = BTreeMap<String, usize>;

/// map of book id to book name
pub type BookIdToName = BTreeMap<usize, String>;

/// - 2D array to check if verse reference is valid
///   - each outer array corresponds to a book of the bible
///   - each inner array corresponds to each chapter of the book
///   - each element of the inner array is the number of verses in that chapter
pub type ReferenceArray = Vec<Vec<usize>>;

/// **NOTE: EVERYTHING IS INDEX 0**
///
/// - 3D array to store content
///   - each outer array corresponds to a book of the bible
///   - each middle array corresponds to each chapter of the book
///   - each inner array corresponds to each verse of the chapter
// pub type BibleContents = Vec<Vec<Vec<String>>>;
pub type BibleContents = Vec<ChapterDataList>;

//

/// This is it's own struct so it will be easier to add things like cross-references later
pub struct VerseData {
    pub(super) content: Option<String>,
}
impl VerseData {}

/// **NOTE: EVERYTHING IS INDEX 0**
// #[derive(Debug)]
pub struct VerseDataList(pub(super) Vec<VerseData>);
impl Deref for VerseDataList {
    type Target = Vec<VerseData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// **NOTE: EVERYTHING IS INDEX 0**
// #[derive(Debug)]
pub struct ChapterDataList(pub(super) Vec<VerseDataList>);
impl Deref for ChapterDataList {
    type Target = Vec<VerseDataList>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// **NOTE: EVERYTHING IS INDEX 0**
// #[derive(Debug)]
pub struct BookDataList(pub(super) Vec<ChapterDataList>);
impl Deref for BookDataList {
    type Target = Vec<ChapterDataList>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct BibleData {
    // pub data: &'a AppData,
    /// regex to match all book names or abbreviations that are part of this data set
    pub book_regex: Regex,
    /// map of abbreviations and actual name (all lowercase) to book id
    pub abbreviations_to_book_id: AbbreviationsToBookId,
    /// map of book id to book name
    pub book_id_to_name: BookIdToName,
    /// - 2D array to check if verse reference is valid
    ///   - each outer array corresponds to a book of the bible
    ///   - each inner array corresponds to each chapter of the book
    ///   - each element of the inner array is the number of verses in that chapter
    pub reference_array: ReferenceArray,
    /// - 3D array to store content
    ///   - each outer array corresponds to a book of the bible
    ///   - each middle array corresponds to each chapter of the book
    ///   - each inner array corresponds to each verse of the chapter
    pub bible_contents: BookDataList,
}

impl BibleData {
    pub fn get_book(&self, book: usize) -> Option<BibleBook> {
        let name = self.book_id_to_name.get(&book)?;
        let chapters = &self.bible_contents.get(book - 1)?;
        Some(BibleBook {
            data: self,
            book,
            name,
            chapters,
        })
    }

    pub fn get_chapter(&self, book: usize, chapter: usize) -> Option<BibleChapter> {
        self.get_book(book)?.get_chapter(chapter)
    }

    pub fn get_verse(&self, book: usize, chapter: usize, verse: usize) -> Option<BibleVerse> {
        self.get_book(book)?.get_chapter(chapter)?.get_verse(verse)
    }

    pub fn get_book_id(&self, book: &str) -> Option<usize> {
        self.abbreviations_to_book_id
            .get(book.to_lowercase().trim_end_matches("."))
            .cloned()
    }
}
