use crate::data::book::BibleBook;

use super::segments::PassageSegments;

pub struct Passage<'a> {
    pub book: BibleBook<'a>,
    pub segments: PassageSegments,
}
