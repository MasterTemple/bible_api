use std::ops::{Deref, DerefMut};

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{
    bible_data::{book::BibleBook, verse::BibleVerse},
    related_media::overlapping_ranges::RangePair,
};

/// - This is a single chapter/verse reference
/// - Ex: `1:2` in `John 1:2`
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ChapterVerse {
    pub chapter: usize,
    pub verse: usize,
}

/// - This is a range of verse references within a single chapter
/// - Ex: `1:2-3` `John 1:2-3`
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ChapterVerseRange {
    pub chapter: usize,
    pub verses: RangePair,
}

/// - This is a range of verse references across a multiple chapters
/// - Ex: `1:2-3:4` in `John 1:2-3:4`
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ChapterRange {
    pub start: ChapterVerse,
    pub end: ChapterVerse,
}

/// Remember, these correspond to
/// ```
///                `Ephesians 1:1-4,5-7,2:2-3:4,6`
///                          |     |   |       | |
///                ----------+     |   |       | |
/// ChapterRange:  `1:1-4`         |   |       | |
///                ----------------+   |       | |
/// ChapterRange:  `1:5-7`             |       | |
///                --------------------+       | |
/// BookRange:     `2:2-3:4`                   | |
///                ----------------------------+ |
/// ChatperVerse:  `3:6`                         |
///                ------------------------------+
/// ```
/// These should be grouped into a single reference
///
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PassageSegment {
    /// - This is a single chapter/verse reference
    /// - Ex: `1:2` in `John 1:2`
    ChapterVerse(ChapterVerse),
    /// - This is a range of verse references within a single chapter
    /// - Ex: `1:2-3` `John 1:2-3`
    ChapterVerseRange(ChapterVerseRange),
    /// - This is a range of verse references across a multiple chapters
    /// - Ex: `John 1:2-3:4`
    ChapterRange(ChapterRange),
}

impl PassageSegment {
    pub fn chapter_verse(chapter: usize, verse: usize) -> Self {
        Self::ChapterVerse(ChapterVerse { chapter, verse })
    }

    pub fn chapter_verse_range(chapter: usize, start_verse: usize, end_verse: usize) -> Self {
        Self::ChapterVerseRange(ChapterVerseRange {
            chapter,
            verses: RangePair {
                start: start_verse,
                end: end_verse,
            },
        })
    }

    pub fn chapter_range(
        start_chapter: usize,
        start_verse: usize,
        end_chapter: usize,
        end_verse: usize,
    ) -> Self {
        Self::ChapterRange(ChapterRange {
            start: ChapterVerse {
                chapter: start_chapter,
                verse: start_verse,
            },
            end: ChapterVerse {
                chapter: end_chapter,
                verse: end_verse,
            },
        })
    }
}

impl PassageSegment {
    pub fn label(&self) -> String {
        match self {
            PassageSegment::ChapterVerse(chapter_verse) => {
                format!("{}:{}", chapter_verse.chapter, chapter_verse.verse)
            }
            PassageSegment::ChapterVerseRange(chapter_range) => {
                format!(
                    "{}:{}-{}",
                    chapter_range.chapter, chapter_range.verses.start, chapter_range.verses.end
                )
            }
            PassageSegment::ChapterRange(book_range) => {
                format!(
                    "{}:{}-{}:{}",
                    book_range.start.chapter,
                    book_range.start.verse,
                    book_range.end.chapter,
                    book_range.end.verse
                )
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PassageSegments(pub Vec<PassageSegment>);

impl PassageSegments {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn try_parse(segment_input: &str) -> Option<Self> {
        let segment_match = POST_BOOK_VALID_REFERENCE_SEGMENT_CHARACTERS
            .find_iter(segment_input)
            .next()?;
        let segments = parse_reference_segments(segment_match.as_str());
        Some(segments)
    }

    /// nobody ought to call this unless their segment_input is validated by the regex
    // pub fn parse(segment_input: &str) -> Self {
    //     parse_reference_segments(segment_input)
    // }

    pub fn label(&self) -> String {
        let mut previous_chapter: Option<usize> = None;
        let mut label_segments: Vec<String> = vec![];
        // let mut label_str = String::new();
        for seg in self.0.iter() {
            let next_seg = match seg {
                PassageSegment::ChapterVerse(chapter_verse) => {
                    if previous_chapter.is_some_and(|prev| prev == chapter_verse.chapter) {
                        format!("{}", chapter_verse.verse)
                    } else {
                        format!("{}:{}", chapter_verse.chapter, chapter_verse.verse)
                    }
                }
                PassageSegment::ChapterVerseRange(chapter_range) => {
                    if previous_chapter.is_some_and(|prev| prev == chapter_range.chapter) {
                        format!(
                            "{}-{}",
                            chapter_range.verses.start, chapter_range.verses.end
                        )
                    } else {
                        format!(
                            "{}:{}-{}",
                            chapter_range.chapter,
                            chapter_range.verses.start,
                            chapter_range.verses.end
                        )
                    }
                }
                PassageSegment::ChapterRange(book_range) => {
                    if previous_chapter.is_some_and(|prev| prev == book_range.start.chapter) {
                        format!(
                            "{}-{}:{}",
                            book_range.start.verse, book_range.end.chapter, book_range.end.verse
                        )
                    } else {
                        format!(
                            "{}:{}-{}:{}",
                            book_range.start.chapter,
                            book_range.start.verse,
                            book_range.end.chapter,
                            book_range.end.verse
                        )
                    }
                }
            };
            let ending_chapter = seg.get_ending_chapter();
            // // if new chapter, add '; '
            // if previous_chapter.is_some_and(|prev| prev != ending_chapter) {
            //     label_segments.push(String::from("; "));
            // }
            // // if same chapter, add ','
            // else {
            //     label_segments.push(String::from(","));
            // }
            if let Some(prev) = previous_chapter {
                match prev == ending_chapter {
                    // if same chapter, add ','
                    true => label_segments.push(String::from(",")),
                    // if new chapter, add '; '
                    false => label_segments.push(String::from("; ")),
                }
            }
            label_segments.push(next_seg);
            previous_chapter = Some(ending_chapter);
        }
        label_segments.join("")
    }
}

impl Deref for PassageSegments {
    type Target = Vec<PassageSegment>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PassageSegments {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PassageSegment {
    pub fn get_starting_verse(&self) -> usize {
        match self {
            PassageSegment::ChapterVerse(chapter_verse) => chapter_verse.verse,
            PassageSegment::ChapterVerseRange(chapter_range) => chapter_range.verses.start,
            PassageSegment::ChapterRange(book_range) => book_range.start.verse,
        }
    }

    pub fn get_starting_chapter(&self) -> usize {
        match self {
            PassageSegment::ChapterVerse(chapter_verse) => chapter_verse.chapter,
            PassageSegment::ChapterVerseRange(chapter_range) => chapter_range.chapter,
            PassageSegment::ChapterRange(book_range) => book_range.start.chapter,
        }
    }

    pub fn get_ending_verse(&self) -> usize {
        match self {
            PassageSegment::ChapterVerse(chapter_verse) => chapter_verse.verse,
            PassageSegment::ChapterVerseRange(chapter_range) => chapter_range.verses.end,
            PassageSegment::ChapterRange(book_range) => book_range.end.verse,
        }
    }

    pub fn get_ending_chapter(&self) -> usize {
        match self {
            PassageSegment::ChapterVerse(chapter_verse) => chapter_verse.chapter,
            PassageSegment::ChapterVerseRange(chapter_range) => chapter_range.chapter,
            PassageSegment::ChapterRange(book_range) => book_range.end.chapter,
        }
    }
}

static POST_BOOK_VALID_REFERENCE_SEGMENT_CHARACTERS: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^ *\d+:\d+( *[,:;\-–] *\d+)*").unwrap());

static NON_SEGMENT_CHARACTERS: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^\d,:;-]+").unwrap());

static TRAILING_NON_DIGITS: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\D+$)").unwrap());

static SEGMENT_SPLITTERS: Lazy<Regex> = Lazy::new(|| Regex::new("(,|;)").unwrap());

/// - This function is meant to parse the `1:1-4,5-7,2:2-3:4,6` in `Ephesians 1:1-4,5-7,2:2-3:4,6`
/// - Don't pass it anything else please :)
/**
Passing `1` will result in
```no_run
[src/main.rs:27:5] parse_reference_segments("1") = [
    ChapterVerse(
        ChapterVerse {
            chapter: 1,
            verse: 1,
        },
    ),
]
```
Passing `1:` will result in
```no_run
[src/main.rs:28:5] parse_reference_segments("1:") = [
    ChapterVerse(
        ChapterVerse {
            chapter: 1,
            verse: 1,
        },
    ),
]
```
*/
fn parse_reference_segments(segment_input: &str) -> PassageSegments {
    // swap weird hyphens with normal dash
    let input = &segment_input.replace("–", "-");
    // input now only contains the following characters: [\d,:;-]
    let input = NON_SEGMENT_CHARACTERS.replace_all(&input, "").to_string();

    // removing trailing non-digits (leading shouldn't exist)
    let input = TRAILING_NON_DIGITS.replace_all(&input, "").to_string();

    // split at , or ; (because there is no uniform standard)
    // now I only have ranges (or a single verse)
    let ranges: Vec<&str> = SEGMENT_SPLITTERS.split(input.as_str()).collect();
    // dbg!(&ranges);

    // ALWAYS UPDATE THE CHAPTER SO I CAN USE IT WHEN ONLY VERSES ARE PROVIDED
    let mut chapter = 1;
    let mut segments: Vec<PassageSegment> = Vec::new();
    for range in ranges {
        // if it is a range
        if let Some((left, right)) = range.split_once("-") {
            match (left.split_once(":"), right.split_once(":")) {
                // `ch1:v1 - ch2:v2`
                (Some((ch1, v1)), Some((ch2, v2))) => {
                    chapter = ch2.parse().unwrap();
                    segments.push(PassageSegment::ChapterRange(ChapterRange {
                        start: ChapterVerse {
                            chapter: ch1.parse().unwrap(),
                            verse: v1.parse().unwrap(),
                        },
                        end: ChapterVerse {
                            chapter,
                            verse: v2.parse().unwrap(),
                        },
                    }));
                }
                // `ch1:v1 - v2`
                (Some((ch1, v1)), None) => {
                    chapter = ch1.parse().unwrap();
                    segments.push(PassageSegment::ChapterVerseRange(ChapterVerseRange {
                        chapter,
                        verses: RangePair {
                            start: v1.parse().unwrap(),
                            end: right.parse().unwrap(),
                        },
                    }));
                }
                // `v1 - ch2:v2`
                (None, Some((ch2, v2))) => {
                    let start_chapter = chapter;
                    chapter = ch2.parse().unwrap();
                    segments.push(PassageSegment::ChapterRange(ChapterRange {
                        start: ChapterVerse {
                            chapter,
                            verse: left.parse().unwrap(),
                        },
                        end: ChapterVerse {
                            chapter,
                            verse: v2.parse().unwrap(),
                        },
                    }));
                }
                // `v1 - v2`
                (None, None) => {
                    segments.push(PassageSegment::ChapterVerseRange(ChapterVerseRange {
                        chapter,
                        verses: RangePair {
                            start: left.parse().unwrap(),
                            end: right.parse().unwrap(),
                        },
                    }))
                }
            };
        }
        // else it is not a range, either `ch:v` or `v`
        else {
            // handle `ch:v`
            if let Some((ch, v)) = range.split_once(":") {
                chapter = ch.parse().unwrap();
                segments.push(PassageSegment::ChapterVerse(ChapterVerse {
                    chapter,
                    verse: v.parse().unwrap(),
                }))
            }
            // handle `v`
            else {
                let v = range.parse().unwrap();
                segments.push(PassageSegment::ChapterVerse(ChapterVerse {
                    chapter,
                    verse: v,
                }))
            }
        }
    }
    PassageSegments(segments)
}
