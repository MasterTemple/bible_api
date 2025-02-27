use std::{collections::BTreeMap, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::api::passage::segments::{PassageSegment, PassageSegments};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WordIndices {
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<usize>,
}

/// This is wrapped in an arc so that way the size is smaller when unused
pub type WordIndicesMap = Arc<BTreeMap<String, WordIndices>>;

/// These are ranges, segments separated by ',' or ';' are separate ranges
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BookPassageRange {
    pub book: usize,
    pub segments: PassageSegments,
    /// This is a map of a translation to a specific word starting and ending index
    #[serde(skip_serializing_if = "Option::is_none")]
    pub words: Option<WordIndicesMap>,
}

/**
This is what actually stores all the content of the related media

> **TODO:** I could use string_interner for the tags and for the word_index keys
but that is a pre-mature optimization
*/
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JSONRelatedMedia {
    pub tags: Vec<String>,
    pub references: Vec<BookPassageRange>,
    pub content: String,
}
