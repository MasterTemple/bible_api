use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use crate::{
    api::passage::segments::PassageSegments,
    bible_data::{bible_data::BibleData, formats::parse::ParseBibleData},
    related_media::related_media::{RelatedMedia, RelatedMediaBookOrganizer},
};

use super::passage::passage::Passage;

// #[derive(Default)]
pub struct ApiData {
    pub(crate) bibles: BTreeMap<String, BibleData>,
    pub(crate) bible: BibleData,
    pub(crate) related_media: RelatedMediaBookOrganizer,
}

pub struct Api<'a, T> {
    data: &'a ApiData,
    _content: T,
}

impl<T> Deref for Api<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self._content
    }
}
impl<T> DerefMut for Api<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self._content
    }
}

/// - This is meant to hold [`BibleData`] but also to provide helpful methods for interacting with it
/// - There will be several other fields contained, but it is primarily for caching/indexing
/// purposes (I think it is more appropriate to put here than on [`BibleData`])
/// - Actually maybe not lol
pub struct BibleAPI(ApiData);

impl Deref for BibleAPI {
    type Target = ApiData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for BibleAPI {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl BibleAPI {
    pub fn load(data: BibleData) -> Self {
        Self(ApiData {
            bibles: Default::default(),
            bible: data,
            related_media: RelatedMediaBookOrganizer::default(),
        })
    }

    pub fn add_media(&mut self, list: Vec<RelatedMedia>) {
        self.related_media.add_related_media(list);
    }

    /// This is meant to parse only 1 reference
    pub fn parse_reference(&self, input: &str) -> Option<Passage> {
        // match book
        let book_match = self.bible.book_regex.find_iter(input).next()?;
        // get id
        // (this should always match though)
        let book_id = self.bible.get_book_id(&book_match.as_str())?;
        let book = self.bible.get_book(book_id)?;

        // match passage reference segments that immediately follow
        let remaining = &input[book_match.end()..];
        let segments = PassageSegments::try_parse(remaining)?;

        let passage = Passage { book, segments };
        Some(passage)
    }

    /// This is meant to fidn and parse all references in an input
    pub fn find_and_parse_all_references(input: &str) -> Option<Vec<Located<Passage>>> {
        // hint: use Self::parse_reference on segment splits
        todo!()
    }
}

pub struct Located<T> {
    // pub char_index: usize,
    pub char_range: CharacterRange,
    /// maybe i will not include this and then line calculations in lsp
    pub lined_range: LineRange,
    pub content: T,
}

pub struct CharacterRange {
    pub start_index: u32,
    pub end_index: u32,
}

pub struct LineRange {
    pub start: LinePosition,
    pub end: LinePosition,
}

pub struct LinePosition {
    pub line: u32,
    pub character: u32,
}
