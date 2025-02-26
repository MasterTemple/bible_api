use std::ops::{Deref, DerefMut};

use once_cell::sync::Lazy;
use regex::Regex;

use crate::data::{book::BibleBook, verse::BibleVerse};

/// - This is a single chapter/verse reference
/// - Ex: `1:2` in `John 1:2`
#[derive(Copy, Clone, Debug)]
pub struct PassageVerse {
    pub chapter: usize,
    pub verse: usize,
}

/// - This is a range of verse references within a single chapter
/// - Ex: `1:2-3` `John 1:2-3`
#[derive(Copy, Clone, Debug)]
pub struct PassageVerseRange {
    pub chapter: usize,
    pub start_verse: usize,
    pub end_verse: usize,
}

// impl<'a> IntoIterator for PassageVerseRange {
//     type Item = BibleVerse<'a>;
//
//     type IntoIter = PassageIterator<'a>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         PassageIterator::new(self.chapter, self.chapter, self.start_verse, self.end_verse)
//     }
// }

/// - This is a range of verse references across a multiple chapters
/// - Ex: `1:2-3:4` in `John 1:2-3:4`
#[derive(Copy, Clone, Debug)]
pub struct PassageChapterRange {
    pub start_chapter: usize,
    pub end_chapter: usize,
    pub start_verse: usize,
    pub end_verse: usize,
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
#[derive(Copy, Clone, Debug)]
pub enum PassageSegment {
    /// - This is a single chapter/verse reference
    /// - Ex: `1:2` in `John 1:2`
    PassageVerse(PassageVerse),
    /// - This is a range of verse references within a single chapter
    /// - Ex: `1:2-3` `John 1:2-3`
    PassageVerseRange(PassageVerseRange),
    /// - This is a range of verse references across a multiple chapters
    /// - Ex: `John 1:2-3:4`
    PassageChapterRange(PassageChapterRange),
}

impl PassageSegment {
    pub fn label(&self) -> String {
        match self {
            PassageSegment::PassageVerse(chapter_verse) => {
                format!("{}:{}", chapter_verse.chapter, chapter_verse.verse)
            }
            PassageSegment::PassageVerseRange(chapter_range) => {
                format!(
                    "{}:{}-{}",
                    chapter_range.chapter, chapter_range.start_verse, chapter_range.end_verse
                )
            }
            PassageSegment::PassageChapterRange(book_range) => {
                format!(
                    "{}:{}-{}:{}",
                    book_range.start_chapter,
                    book_range.start_verse,
                    book_range.end_chapter,
                    book_range.end_verse
                )
            }
        }
    }
}

#[derive(Clone, Debug)]
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
                PassageSegment::PassageVerse(chapter_verse) => {
                    if previous_chapter.is_some_and(|prev| prev == chapter_verse.chapter) {
                        format!("{}", chapter_verse.verse)
                    } else {
                        format!("{}:{}", chapter_verse.chapter, chapter_verse.verse)
                    }
                }
                PassageSegment::PassageVerseRange(chapter_range) => {
                    if previous_chapter.is_some_and(|prev| prev == chapter_range.chapter) {
                        format!("{}-{}", chapter_range.start_verse, chapter_range.end_verse)
                    } else {
                        format!(
                            "{}:{}-{}",
                            chapter_range.chapter,
                            chapter_range.start_verse,
                            chapter_range.end_verse
                        )
                    }
                }
                PassageSegment::PassageChapterRange(book_range) => {
                    if previous_chapter.is_some_and(|prev| prev == book_range.start_chapter) {
                        format!(
                            "{}-{}:{}",
                            book_range.start_verse, book_range.end_chapter, book_range.end_verse
                        )
                    } else {
                        format!(
                            "{}:{}-{}:{}",
                            book_range.start_chapter,
                            book_range.start_verse,
                            book_range.end_chapter,
                            book_range.end_verse
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
            PassageSegment::PassageVerse(chapter_verse) => chapter_verse.verse,
            PassageSegment::PassageVerseRange(chapter_range) => chapter_range.start_verse,
            PassageSegment::PassageChapterRange(book_range) => book_range.start_verse,
        }
    }

    pub fn get_starting_chapter(&self) -> usize {
        match self {
            PassageSegment::PassageVerse(chapter_verse) => chapter_verse.chapter,
            PassageSegment::PassageVerseRange(chapter_range) => chapter_range.chapter,
            PassageSegment::PassageChapterRange(book_range) => book_range.start_chapter,
        }
    }

    pub fn get_ending_verse(&self) -> usize {
        match self {
            PassageSegment::PassageVerse(chapter_verse) => chapter_verse.verse,
            PassageSegment::PassageVerseRange(chapter_range) => chapter_range.end_verse,
            PassageSegment::PassageChapterRange(book_range) => book_range.end_verse,
        }
    }

    pub fn get_ending_chapter(&self) -> usize {
        match self {
            PassageSegment::PassageVerse(chapter_verse) => chapter_verse.chapter,
            PassageSegment::PassageVerseRange(chapter_range) => chapter_range.chapter,
            PassageSegment::PassageChapterRange(book_range) => book_range.end_chapter,
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
                    segments.push(PassageSegment::PassageChapterRange(PassageChapterRange {
                        start_chapter: ch1.parse().unwrap(),
                        end_chapter: chapter,
                        start_verse: v1.parse().unwrap(),
                        end_verse: v2.parse().unwrap(),
                    }));
                }
                // `ch1:v1 - v2`
                (Some((ch1, v1)), None) => {
                    chapter = ch1.parse().unwrap();
                    segments.push(PassageSegment::PassageVerseRange(PassageVerseRange {
                        chapter,
                        start_verse: v1.parse().unwrap(),
                        end_verse: right.parse().unwrap(),
                    }));
                }
                // `v1 - ch2:v2`
                (None, Some((ch2, v2))) => {
                    let start_chapter = chapter;
                    chapter = ch2.parse().unwrap();
                    segments.push(PassageSegment::PassageChapterRange(PassageChapterRange {
                        start_chapter,
                        end_chapter: chapter,
                        start_verse: left.parse().unwrap(),
                        end_verse: v2.parse().unwrap(),
                    }));
                }
                // `v1 - v2`
                (None, None) => {
                    segments.push(PassageSegment::PassageVerseRange(PassageVerseRange {
                        chapter,
                        start_verse: left.parse().unwrap(),
                        end_verse: right.parse().unwrap(),
                    }))
                }
            };
        }
        // else it is not a range, either `ch:v` or `v`
        else {
            // handle `ch:v`
            if let Some((ch, v)) = range.split_once(":") {
                chapter = ch.parse().unwrap();
                segments.push(PassageSegment::PassageVerse(PassageVerse {
                    chapter,
                    verse: v.parse().unwrap(),
                }))
            }
            // handle `v`
            else {
                let v = range.parse().unwrap();
                segments.push(PassageSegment::PassageVerse(PassageVerse {
                    chapter,
                    verse: v,
                }))
            }
        }
    }
    PassageSegments(segments)
}
