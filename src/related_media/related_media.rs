use std::{collections::BTreeMap, path::Path, rc::Rc};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::api::passage::segments::PassageSegment;

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

/// Related media is organized by book
pub type RelatedMediaBookOrganizer = BTreeMap<usize, RelatedMediaBook>;

/**
This is references to all the related media for a book
*/
pub struct RelatedMediaBook {
    // chapter:verse (Map<chapter, Map<verse, Vec<ref>>>)
    chapter_verse: BTreeMap<usize, BTreeMap<usize, Vec<RelatedMediaRef>>>,
    // chapter:start_verse-end_verse (Map<chapter, Map<(start_verse, end_verse), ref>>)
    chapter_verse_range: BTreeMap<usize, OverlapMap<RangePair, Vec<RelatedMediaRef>>>,
    // start_chapter:start_verse-end_chapter:end_verse
    chapter_range: OverlapMap<ChapterRangePair, Vec<RelatedMediaRef>>,
}

pub struct RelatedMediaProximity<'a> {
    related_media: &'a Vec<RelatedMediaRef>,
    proximity: PassageSegment,
}

impl RelatedMediaBook {
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
