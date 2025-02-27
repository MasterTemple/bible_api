use std::{collections::BTreeMap, path::Path, rc::Rc};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::api::passage::segments::{
    ChapterRange, ChapterVerse, ChapterVerseRange, PassageSegment,
};

use super::{
    formats::json::JSONRelatedMedia,
    overlapping_ranges::{ChapterRangePair, OverlapMap, RangePair},
};

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct RelatedMedia {
//     pub tags: Vec<String>,
//     pub references: Vec<PassageRange>,
//     pub content: String,
// }

pub type RelatedMedia = JSONRelatedMedia;

pub type RelatedMediaRef = Rc<RelatedMedia>;

pub trait MapExtensions<K, V> {
    fn get_or_insert(&mut self, key: &K) -> &V;
    fn get_or_insert_mut(&mut self, key: &K) -> &mut V;
}

impl<K: Ord + Clone, V: Default> MapExtensions<K, V> for BTreeMap<K, V> {
    fn get_or_insert(&mut self, key: &K) -> &V {
        if !self.contains_key(key) {
            _ = self.insert(key.clone(), V::default());
        }
        self.get(key).unwrap()
    }

    fn get_or_insert_mut(&mut self, key: &K) -> &mut V {
        if !self.contains_key(key) {
            _ = self.insert(key.clone(), V::default());
        }
        self.get_mut(key).unwrap()
    }
}

/// Related media is organized by book
#[derive(Default)]
pub struct RelatedMediaBookOrganizer(BTreeMap<usize, RelatedMediaBook>);
impl RelatedMediaBookOrganizer {
    // do i want to take BookPassageRange (i probably want something like this) or book and then PassageSegment?
    pub fn get_related_media(
        &self,
        book: usize,
        passage_segment: PassageSegment,
    ) -> Option<Vec<RelatedMediaProximity>> {
        let media_book = self.0.get(&book)?;
        media_book.get_passage_media(passage_segment)
    }

    pub fn add_related_media(&mut self, list: Vec<RelatedMedia>) {
        for item in list {
            let rc_item = Rc::new(item);
            for reference in rc_item.references.iter() {
                let book = reference.book;
                let media_book = self.0.get_or_insert_mut(&book);
                for seg in reference.segments.0.iter() {
                    match seg {
                        PassageSegment::ChapterVerse(ChapterVerse { chapter, verse }) => {
                            let chapter_map = media_book.chapter_verse.get_or_insert_mut(&chapter);
                            let list = chapter_map.get_or_insert_mut(&verse);
                            list.push(rc_item.clone());
                        }
                        PassageSegment::ChapterVerseRange(ChapterVerseRange {
                            chapter,
                            verses,
                        }) => {
                            let chapter_map =
                                media_book.chapter_verse_range.get_or_insert_mut(&chapter);
                            let list = chapter_map.get_or_insert_mut(&verses);
                            list.push(rc_item.clone());
                        }
                        PassageSegment::ChapterRange(ChapterRange { start, end }) => {
                            let chapter_range_pair = ChapterRangePair::new(
                                start.chapter,
                                start.verse,
                                end.chapter,
                                end.verse,
                            );
                            let list = media_book
                                .chapter_range
                                .get_or_insert_mut(&chapter_range_pair);
                            list.push(rc_item.clone())
                        }
                    };
                }
            }
        }
    }
}

/**
This is references to all the related media for a book
*/
#[derive(Default)]
pub struct RelatedMediaBook {
    // chapter:verse (Map<chapter, Map<verse, Vec<ref>>>)
    chapter_verse: BTreeMap<usize, BTreeMap<usize, Vec<RelatedMediaRef>>>,
    // chapter:start_verse-end_verse (Map<chapter, Map<(start_verse, end_verse), ref>>)
    chapter_verse_range: BTreeMap<usize, OverlapMap<RangePair, Vec<RelatedMediaRef>>>,
    // start_chapter:start_verse-end_chapter:end_verse
    chapter_range: OverlapMap<ChapterRangePair, Vec<RelatedMediaRef>>,
}

#[derive(Debug)]
pub struct RelatedMediaProximity<'a> {
    related_media: &'a Vec<RelatedMediaRef>,
    proximity: PassageSegment,
}

impl RelatedMediaBook {
    pub fn get_passage_media(
        &self,
        passage_segment: PassageSegment,
    ) -> Option<Vec<RelatedMediaProximity<'_>>> {
        match passage_segment {
            PassageSegment::ChapterVerse(ChapterVerse { chapter, verse }) => self
                .get_chapter_verse_media(chapter, verse)
                .map(|it| vec![it]),
            PassageSegment::ChapterVerseRange(ChapterVerseRange { chapter, verses }) => {
                self.get_chapter_verse_range_media(chapter, verses.start, verses.end)
            }
            PassageSegment::ChapterRange(ChapterRange { start, end }) => {
                self.get_chapter_range_media(start.chapter, start.verse, end.chapter, end.verse)
            }
        }
    }

    pub fn get_chapter_verse_media(
        &self,
        chapter: usize,
        verse: usize,
        // Option<&Vec<RelatedMediaRef>>
    ) -> Option<RelatedMediaProximity<'_>> {
        self.chapter_verse
            .get(&chapter)?
            .get(&verse)
            .map(|related_media| RelatedMediaProximity {
                related_media,
                proximity: PassageSegment::chapter_verse(chapter, verse),
            })
    }

    pub fn get_chapter_verse_range_media(
        &self,
        chapter: usize,
        start_verse: usize,
        end_verse: usize,
    ) -> Option<Vec<RelatedMediaProximity<'_>>> {
        let range_pair = RangePair::new(start_verse, end_verse);
        let overlapping = self
            .chapter_verse_range
            .get(&chapter)?
            .iter_overlapping(range_pair)
            .map(|(key, related_media)| RelatedMediaProximity {
                related_media,
                proximity: PassageSegment::chapter_verse_range(chapter, key.start, key.end),
            })
            .collect_vec();
        Some(overlapping)
    }

    pub fn get_chapter_range_media(
        &self,
        start_chapter: usize,
        start_verse: usize,
        end_chapter: usize,
        end_verse: usize,
    ) -> Option<Vec<RelatedMediaProximity<'_>>> {
        let range_pair = ChapterRangePair::new(start_chapter, start_verse, end_chapter, end_verse);
        let overlapping = self
            .chapter_range
            .iter_overlapping(range_pair)
            .map(|(key, related_media)| RelatedMediaProximity {
                related_media,
                proximity: PassageSegment::chapter_range(
                    key.start_chapter,
                    key.start_verse,
                    key.end_chapter,
                    key.end_verse,
                ),
            })
            .collect_vec();
        Some(overlapping)
    }
}

#[test]
fn import_related_media() {
    let content =
        std::fs::read_to_string(Path::new("/home/dglinuxtemple/related_media.json")).unwrap();
    let related_media: Vec<RelatedMedia> = serde_json::from_str(&content).unwrap();
    dbg!(&related_media);
}
