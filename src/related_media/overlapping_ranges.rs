use std::collections::BTreeMap;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

pub trait OverlapsWith {
    fn overlaps_with(&self, other: &Self) -> bool;
}

pub struct OverlapMap<K: Ord + OverlapsWith, V>(BTreeMap<K, V>);
impl<K: Ord + OverlapsWith, V> OverlapMap<K, V> {
    pub fn iter_overlapping(&self, this: K) -> impl Iterator<Item = (&K, &V)> {
        // I could maybe do some heuristics to search through less, but this is fine for
        // now
        self.0
            .iter()
            .filter(move |(key, _)| this.overlaps_with(key))
    }

    pub fn get_overlapping(&self, this: K) -> Vec<(&K, &V)> {
        // I could maybe do some heuristics to search through less, but this is fine for
        // now
        // self.0
        //     .iter()
        //     .filter(|(key, _)| this.overlaps_with(key))
        self.iter_overlapping(this).collect_vec()
    }

    pub fn get_overlapping_optional(&self, this: K) -> Option<Vec<(&K, &V)>> {
        let results = self.get_overlapping(this);
        if results.len() == 0 {
            None
        } else {
            Some(results)
        }
    }

    // pub fn get_overlapping_values(&self, this: K) -> Option<Vec<&V>> {
    //     // I could maybe do some heuristics to search through less, but this is fine for
    //     // now
    //     let results = self
    //         .0
    //         .iter()
    //         .filter(|(key, _)| this.overlaps_with(key))
    //         .map(|(_, value)| value)
    //         .collect_vec();
    //     if results.len() == 0 {
    //         None
    //     } else {
    //         Some(results)
    //     }
    // }
}

/// Could be used for a chapter or verse range
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RangePair {
    pub start: usize,
    pub end: usize,
}

impl RangePair {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn from_verse(verse: usize) -> Self {
        Self {
            start: verse,
            end: verse,
        }
    }
}

impl OverlapsWith for RangePair {
    fn overlaps_with(&self, other: &Self) -> bool {
        // checking overlap by checking if there is space between their edges
        !(
            // other ends before this starts (which is also this starts before other ends)
            other.end < self.start
            // other starts after this ends (which is also this ends after other starts)
            || other.start > self.end
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChapterRangePair {
    pub(super) start_chapter: usize,
    pub(super) end_chapter: usize,
    pub(super) start_verse: usize,
    pub(super) end_verse: usize,
}

impl ChapterRangePair {
    pub fn new(
        start_chapter: usize,
        start_verse: usize,
        end_chapter: usize,
        end_verse: usize,
    ) -> Self {
        Self {
            start_chapter,
            end_chapter,
            start_verse,
            end_verse,
        }
    }

    pub fn from_chapter_verse(chapter: usize, verse: usize) -> Self {
        Self {
            start_chapter: chapter,
            end_chapter: chapter,
            start_verse: verse,
            end_verse: verse,
        }
    }
}

impl OverlapsWith for ChapterRangePair {
    fn overlaps_with(&self, other: &Self) -> bool {
        // checking overlap by checking if there is space between their edges
        !(
            // other ends before this starts (which is also this starts before other ends)
            other.end_chapter < self.start_chapter
            || (other.end_chapter == self.start_chapter && other.end_verse < self.start_verse)
            // other starts after this ends (which is also this ends after other starts)
            || other.start_chapter > self.end_chapter
                || (other.start_chapter == self.end_chapter && other.start_verse > self.end_verse)
        )
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_range_pairs() {
        // 3-5
        let this = RangePair::new(3, 5);
        // overlaps 1-2: false
        assert_eq!(this.overlaps_with(&RangePair::new(1, 2)), false);
        // overlaps 1-3: true
        assert_eq!(this.overlaps_with(&RangePair::new(1, 3)), true);
        // overlaps 1-4: true
        assert_eq!(this.overlaps_with(&RangePair::new(1, 4)), true);
        // overlaps 2-3: true
        assert_eq!(this.overlaps_with(&RangePair::new(2, 3)), true);
        // overlaps 3-4: true
        assert_eq!(this.overlaps_with(&RangePair::new(3, 4)), true);
        // overlaps 4-5: true
        assert_eq!(this.overlaps_with(&RangePair::new(4, 5)), true);
        // overlaps 5-7: true
        assert_eq!(this.overlaps_with(&RangePair::new(5, 7)), true);
        // overlaps 6-7: true
        assert_eq!(this.overlaps_with(&RangePair::new(6, 7)), false);

        // 3-3
        let this = RangePair::new(3, 3);
        // overlaps 1-2: false
        assert_eq!(this.overlaps_with(&RangePair::new(1, 2)), false);
        // overlaps 1-3: true
        assert_eq!(this.overlaps_with(&RangePair::new(1, 3)), true);
        // overlaps 2-4: true
        assert_eq!(this.overlaps_with(&RangePair::new(2, 4)), true);
        // overlaps 3-3: true
        assert_eq!(this.overlaps_with(&RangePair::new(3, 3)), true);
        // overlaps 3-4: true
        assert_eq!(this.overlaps_with(&RangePair::new(3, 4)), true);
        // overlaps 4-5: false
        assert_eq!(this.overlaps_with(&RangePair::new(4, 5)), false);
    }

    #[test]
    fn test_chapter_range_pairs() {
        // 2:3-4:5
        let this = ChapterRangePair::new(2, 3, 4, 5);
        // overlaps 1:1-1:2 = false
        assert_eq!(
            this.overlaps_with(&ChapterRangePair::new(1, 1, 1, 2)),
            false
        );
        // overlaps 1:1-2:2 = false
        assert_eq!(
            this.overlaps_with(&ChapterRangePair::new(1, 1, 2, 2)),
            false
        );
        // overlaps 1:1-2:3 = true
        assert_eq!(this.overlaps_with(&ChapterRangePair::new(1, 1, 2, 3)), true);
        //
        // overlaps 2:1-2:3 = true
        assert_eq!(this.overlaps_with(&ChapterRangePair::new(2, 1, 2, 3)), true);
        // overlaps 2:3-2:4 = true
        assert_eq!(this.overlaps_with(&ChapterRangePair::new(2, 3, 2, 4)), true);
        // overlaps 2:3-3:1 = true
        assert_eq!(this.overlaps_with(&ChapterRangePair::new(2, 3, 3, 1)), true);
        // overlaps 3:1-4:1 = true
        assert_eq!(this.overlaps_with(&ChapterRangePair::new(3, 1, 4, 1)), true);
        // overlaps 2:3-4:1 = true
        assert_eq!(this.overlaps_with(&ChapterRangePair::new(2, 3, 4, 1)), true);
        // overlaps 2:3-4:6 = true
        assert_eq!(this.overlaps_with(&ChapterRangePair::new(2, 3, 4, 6)), true);
        // overlaps 2:3-4:6 = true
        assert_eq!(this.overlaps_with(&ChapterRangePair::new(2, 3, 4, 6)), true);
        // overlaps 4:6-4:7 = false
        assert_eq!(
            this.overlaps_with(&ChapterRangePair::new(4, 6, 4, 7)),
            false
        );
        // overlaps 4:7-5:1 = false
        assert_eq!(
            this.overlaps_with(&ChapterRangePair::new(4, 7, 5, 1)),
            false
        );

        // 3:3-3:3
        let this = ChapterRangePair::new(3, 3, 3, 3);
        // overlaps 1:1-2:2 = false
        assert_eq!(
            this.overlaps_with(&ChapterRangePair::new(1, 1, 2, 2)),
            false
        );
        // overlaps 2:3-4:6 = true
        assert_eq!(this.overlaps_with(&ChapterRangePair::new(2, 3, 4, 6)), true);
        // overlaps 3:2-3:3 = true
        assert_eq!(this.overlaps_with(&ChapterRangePair::new(3, 2, 3, 3)), true);
        // overlaps 3:3-3:3 = true
        assert_eq!(this.overlaps_with(&ChapterRangePair::new(3, 3, 3, 3)), true);
        // overlaps 3:3-3:4 = true
        assert_eq!(this.overlaps_with(&ChapterRangePair::new(3, 3, 3, 4)), true);
        // overlaps 4:4-5:5 = false
        assert_eq!(
            this.overlaps_with(&ChapterRangePair::new(4, 4, 5, 5)),
            false
        );
    }
}
