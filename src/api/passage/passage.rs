use itertools::Itertools;

use crate::data::book::BibleBook;

use super::{
    formatting_template::{
        ChapterFormatParameters, FormattingTemplate, PassageFormatParameters, PassageFormatter,
        SegmentFormatParameters, VerseFormatParameters,
    },
    segments::PassageSegments,
};

#[derive(Clone)]
pub struct Passage<'a> {
    pub book: BibleBook<'a>,
    pub segments: PassageSegments,
}

impl Passage<'_> {
    // pub fn format2(&self, formatter: &PassageFormatter) -> String {
    //     let segment_template = FormattingTemplate::from_template(&formatter.segment).unwrap();
    //     let chapter_template = FormattingTemplate::from_template(&formatter.chapter).unwrap();
    //     let verse_template = FormattingTemplate::from_template(&formatter.verse).unwrap();
    //     let passage_template = FormattingTemplate::from_template(&formatter.text).unwrap();
    //
    //     let book = self.book.get_name();
    //
    //     let mut text = String::new();
    //     for seg in self.iter_segments() {
    //         let prev_chapter: Option<usize> = None;
    //         for bible_verse in seg {
    //             // if prev_chapter.is_some_and(|prev| prev == verse.chapter_number()) {
    //             //
    //             // }
    //             let Some(content) = bible_verse.get_content() else {
    //                 continue;
    //             };
    //             let chapter = bible_verse.chapter_number();
    //             let verse = bible_verse.verse_number();
    //             let params = VerseFormatParameters {
    //                 book,
    //                 chapter,
    //                 verse,
    //                 content,
    //             };
    //             let filled = verse_template.fill(&params).unwrap();
    //             text.push_str(&filled);
    //         }
    //     }
    //     todo!()
    // }
    //
    // pub fn format2(&self, formatter: &PassageFormatter) -> String {
    //     let segment_template = FormattingTemplate::from_template(&formatter.segment).unwrap();
    //     let chapter_template = FormattingTemplate::from_template(&formatter.chapter).unwrap();
    //     let verse_template = FormattingTemplate::from_template(&formatter.verse).unwrap();
    //     let passage_template = FormattingTemplate::from_template(&formatter.text).unwrap();
    //
    //     let book = self.book.get_name();
    //
    //     self.iter_segments()
    //         .map(|seg| {
    //             let verses = &seg
    //                 .filter_map(|bible_verse| {
    //                     let content = bible_verse.get_content()?;
    //                     let chapter = bible_verse.chapter_number();
    //                     let verse = bible_verse.verse_number();
    //                     let params = VerseFormatParameters {
    //                         book,
    //                         chapter,
    //                         verse,
    //                         content,
    //                     };
    //                     let filled = verse_template.fill(&params).unwrap();
    //                     Some(filled)
    //                 })
    //                 .join(&formatter.join_verses);
    //
    //             let params = SegmentFormatParameters {
    //                 book,
    //                 label: &seg.label(),
    //                 verses,
    //             };
    //             segment_template.fill(&params).unwrap()
    //         })
    //         .join(&formatter.join_segments)
    //     // todo!()
    // }

    /**
    Returns text like the following:

    ```text
    [1:1] Paul, an apostle of Christ Jesus by the will of God, To the saints who are in Ephesus, and are faithful in Christ Jesus:
    [1:2] Grace to you and peace from God our Father and the Lord Jesus Christ.
    [1:3] Blessed be the God and Father of our Lord Jesus Christ, who has blessed us in Christ with every spiritual blessing in the heavenly places,
    [1:4] even as he chose us in him before the foundation of the world, that we should be holy and blameless before him. In love
    ```
    */
    pub fn format(&self, formatter: &PassageFormatter) -> String {
        // let book = self.book;
        let book = self.book.get_name();
        let segment_template = FormattingTemplate::from_template(&formatter.segment).unwrap();
        let chapter_template = FormattingTemplate::from_template(&formatter.chapter).unwrap();
        let verse_template = FormattingTemplate::from_template(&formatter.verse).unwrap();
        let passage_template = FormattingTemplate::from_template(&formatter.text).unwrap();

        let segment_range_content = self
            .segments
            .iter()
            .map(|seg| {
                let chapter_range_content = (seg.get_starting_chapter()..=seg.get_ending_chapter())
                    .map(|chapter| {
                        let start_verse = if chapter == seg.get_starting_chapter() {
                            seg.get_starting_verse()
                        } else {
                            1
                        };
                        let end_verse = if chapter == seg.get_ending_chapter() {
                            seg.get_ending_verse()
                        } else {
                            self.book.get_chapter(chapter).unwrap().verse_count()
                        };
                        let verse_range_content = (start_verse..=end_verse)
                            .filter_map(|verse| {
                                let content =
                                    &self.book.get_verse(chapter, verse)?.get_content()?;
                                let params = VerseFormatParameters {
                                    book,
                                    chapter,
                                    verse,
                                    content,
                                };
                                Some(verse_template.fill(&params).unwrap())
                            })
                            .collect::<Vec<_>>()
                            .join(&formatter.join_verses);

                        let params = ChapterFormatParameters {
                            book,
                            chapter,
                            start_verse,
                            end_verse,
                            verses: &verse_range_content,
                        };
                        chapter_template.fill(&params).unwrap()
                    })
                    .collect::<Vec<_>>()
                    // this will not work how you expect
                    // because segments are outside of chapters
                    .join(&formatter.join_chapters);
                let params = SegmentFormatParameters {
                    book,
                    label: &seg.label(),
                    verses: &chapter_range_content,
                };
                segment_template.fill(&params).unwrap()
            })
            .collect::<Vec<_>>()
            .join(&formatter.join_segments);

        let params = PassageFormatParameters {
            book,
            segments: &segment_range_content,
            label: &self.segments.label(),
        };
        passage_template.fill(&params).unwrap()
    }
}
