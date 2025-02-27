use serde::Serialize;

use crate::api::bible_api::BibleAPI;

use super::{
    bible_data::{BibleData, ChapterDataList, VerseDataList},
    book::BibleBook,
    verse::BibleVerse,
};

#[derive(Clone, Copy, Serialize)]
pub struct BibleChapter<'a> {
    // pub(super) api: &'a BibleAPI,
    #[serde(skip)]
    pub(super) data: &'a BibleData,
    pub(super) book: usize,
    pub(super) chapter: usize,
    pub(super) verses: &'a VerseDataList,
}

impl<'a> BibleChapter<'a> {
    pub fn chapter_data(&self) -> &'a ChapterDataList {
        &self.data.bible_contents[self.book]
    }
}

impl<'a> BibleChapter<'a> {
    pub fn get_book(&self) -> BibleBook<'a> {
        self.data.get_book(self.book).unwrap()
    }

    pub fn chapter_number(&self) -> usize {
        self.chapter
    }

    pub fn verse_count(&self) -> usize {
        self.verses.len()
    }

    pub fn get_verse(&self, verse: usize) -> Option<BibleVerse<'a>> {
        // the early return from `verses.get()?` is for when the Bible verse does not exist
        // within the verse list, NOT for when the Bible verse's content has been deemed
        // not authentic to the original texts
        let content = self.verses.get(verse - 1)?.content.as_deref();
        Some(BibleVerse {
            data: self.data,
            // api: self.api,
            book: self.book,
            chapter: self.chapter,
            verse,
            content,
        })
    }
}
