use crate::data::{book::BibleBook, verse::BibleVerse};

use super::passage::Passage;

pub struct PassageSegmentIterator<'a> {
    book: BibleBook<'a>,
    /// also serves as start chapter
    current_chapter: usize,
    /// also serves as start verse
    current_verse: usize,
    /// Inclusive
    end_chapter: usize,
    /// Inclusive
    end_verse: usize,
}

impl<'a> PassageSegmentIterator<'a> {
    pub fn new(
        book: BibleBook<'a>,
        start_chapter: usize,
        end_chapter: usize,
        start_verse: usize,
        end_verse: usize,
    ) -> Self {
        Self {
            book,
            current_chapter: start_chapter,
            current_verse: start_verse,
            end_chapter,
            end_verse,
        }
    }
}

impl<'a> Iterator for PassageSegmentIterator<'a> {
    type Item = BibleVerse<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // if beyond appropriate chapter, finish
        if self.current_chapter > self.end_chapter {
            return None;
        }

        // if beyond appropriate verse (when on last chapter), finish
        if self.current_chapter == self.end_chapter && self.current_verse > self.end_verse {
            return None;
        }

        let chapter = self.book.get_chapter(self.current_chapter)?;
        let verse = chapter.get_verse(self.current_verse);

        // advance state (to next valid verse)
        self.current_verse += 1;
        if self.current_verse > chapter.verse_count() {
            // start from the beginning of the next chapter
            self.current_chapter += 1;
            self.current_verse = 1;
        }

        verse
    }
}

pub struct PassageIterator<'a> {
    passage: Passage<'a>,
    segment_index: usize,
    segment_iterator: Option<PassageSegmentIterator<'a>>,
}

impl<'a> IntoIterator for Passage<'a> {
    type Item = BibleVerse<'a>;

    type IntoIter = PassageIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PassageIterator {
            passage: self,
            segment_index: 0,
            segment_iterator: None,
        }
    }
}

impl<'a> Iterator for PassageIterator<'a> {
    type Item = BibleVerse<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // get the next element and return it
        let next = self.segment_iterator.as_mut().and_then(|iter| iter.next());
        if next.is_some() {
            return next;
        }

        // otherwise, when the child iterator is exhausted (or it was never set)
        // set the segment_iterator
        let segment = self.passage.segments.get(self.segment_index)?;
        // update index for next time because i am starting uninitialized
        self.segment_index += 1;
        let mut segment_iterator = PassageSegmentIterator::new(
            self.passage.book,
            segment.get_starting_chapter(),
            segment.get_ending_chapter(),
            segment.get_starting_verse(),
            segment.get_ending_verse(),
        );

        let next = segment_iterator.next();
        self.segment_iterator = Some(segment_iterator);
        return next;
    }
}
