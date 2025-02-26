use super::{
    bible_data::{BibleData, ChapterDataList, VerseDataList},
    chapter::BibleChapter,
    verse::BibleVerse,
};

#[derive(Clone, Copy)]
pub struct BibleBook<'a> {
    pub(super) data: &'a BibleData,
    pub(super) book: usize,
    pub(super) name: &'a str,
    /// **NOTE: EVERYTHING IS INDEX 0**
    pub(super) chapters: &'a ChapterDataList,
}

impl<'a> BibleBook<'a> {
    pub fn book_number(&self) -> usize {
        self.book
    }

    // when i add lifetime 'a to self, then it doesn't work
    // why? because &'a creates a new lifetime and isn't using the one it already has
    pub fn get_chapter(&self, chapter: usize) -> Option<BibleChapter<'a>> {
        let verses: &'a VerseDataList = self.chapters.get(chapter - 1)?;
        Some(BibleChapter {
            data: self.data,
            book: self.book,
            chapter,
            verses,
        })
    }

    pub fn get_verse(&self, chapter: usize, verse: usize) -> Option<BibleVerse<'a>> {
        self.get_chapter(chapter)?.get_verse(verse)
    }

    pub fn get_name(&self) -> &'a str {
        self.name
    }
}
