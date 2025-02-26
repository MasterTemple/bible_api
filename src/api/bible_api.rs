use crate::{api::passage::segments::PassageSegments, data::bible_data::BibleData};

use super::passage::passage::Passage;

/// - This is meant to hold [`BibleData`] but also to provide helpful methods for interacting with it
/// - There will be several other fields contained, but it is primarily for caching/indexing
/// purposes (I think it is more appropriate to put here than on [`BibleData`])
/// - Actually maybe not lol
pub struct BibleAPI {
    data: BibleData,
    // /// This is dynamically generated for several reasons
    // /// - I am guaranteed that I only match on books that will return a result
    // book_regex: Regex,
}

impl BibleAPI {
    pub fn load(data: BibleData) -> Self {
        Self { data }
    }

    /// This is meant to parse only 1 reference
    pub fn parse_reference(&self, input: &str) -> Option<Passage> {
        // match book
        let book_match = self.data.book_regex.find_iter(input).next()?;
        // get id
        // (this should always match though)
        let book_id = self.data.get_book_id(&book_match.as_str())?;
        let book = self.data.get_book(book_id)?;

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
