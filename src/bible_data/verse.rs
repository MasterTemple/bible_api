use serde::{Deserialize, Serialize};

use crate::{
    api::{
        bible_api::{Api, ApiData, BibleAPI},
        passage::segments::{ChapterVerse, PassageSegment},
    },
    related_media::related_media::{RelatedMedia, RelatedMediaProximity},
};

use super::{bible_data::BibleData, book::BibleBook, chapter::BibleChapter};

#[derive(Clone, Copy, Serialize)]
pub struct BibleVerse<'a> {
    // pub(super) data: &'a ApiData,
    #[serde(skip)]
    pub(super) bible: &'a BibleData,
    pub(super) book: usize,
    pub(super) chapter: usize,
    pub(super) verse: usize,
    /**
    This is because some "valid" references don't have content.
    Here are some examples from ESV:
    - Matthew 12:47
    - Matthew 17:21
    - Matthew 18:11
    - Matthew 23:14
    - Mark 7:16
    - Mark 9:44
    - Mark 9:46
    - Mark 11:26
    - Mark 15:28
    - Luke 17:36
    - Luke 23:17
    - John 5:4
    - Acts 8:37
    - Acts 15:34
    - Acts 24:7
    - Acts 28:29
    - Romans 16:24
    */
    pub(super) content: Option<&'a str>,
}

impl<'a> BibleVerse<'a> {
    pub fn get_book(&self) -> BibleBook<'a> {
        self.bible.get_book(self.book).unwrap()
    }

    pub fn get_chapter(&self) -> BibleChapter<'a> {
        self.get_book().get_chapter(self.chapter).unwrap()
    }

    pub fn chapter_number(&self) -> usize {
        self.chapter
    }

    pub fn verse_number(&self) -> usize {
        self.verse
    }

    pub fn get_content(&self) -> Option<&'a str> {
        self.content
    }

    // pub fn get_related_media(&self) -> Option<Vec<RelatedMediaProximity>> {
    //     self.api.related_media.get_related_media(
    //         self.book,
    //         PassageSegment::ChapterVerse(ChapterVerse {
    //             chapter: self.chapter,
    //             verse: self.verse,
    //         }),
    //     )
    // }
}

impl Api<'_, BibleVerse<'_>> {
    pub fn get_related_media(&self) -> Option<Vec<RelatedMediaProximity>> {
        self.api.related_media.get_related_media(
            self.book,
            PassageSegment::ChapterVerse(ChapterVerse {
                chapter: self.chapter,
                verse: self.verse,
            }),
        )
    }
}
