#[derive(Clone, Debug)]
enum TemplateSegment {
    Raw(String),
    Variable(String),
}

#[derive(Clone, Debug)]
pub struct FormattingTemplate(Vec<TemplateSegment>);

impl FormattingTemplate {
    pub fn from_template(template: &str) -> Result<Self, String> {
        let mut prev: Option<char> = None;
        let indices = template
            .char_indices()
            .filter(|(_, ch)| {
                let is_brace = *ch == '{' || *ch == '}';
                let is_escaped = prev.is_some_and(|ch| ch == '\\');
                prev = Some(*ch);
                is_brace && !is_escaped
            })
            .collect::<Vec<_>>();

        let mut segments = Vec::new();
        let mut output = String::new();
        let mut prev = None;
        for (idx, win) in indices.windows(2).enumerate() {
            if idx % 2 == 1 {
                continue;
            }
            let left = win[0];
            let right = win[1];
            if left.1 != '{' || right.1 != '}' {
                return Err(format!("Improperly formatted template"));
            }
            let content = &template[prev.unwrap_or(0)..left.0];
            output.push_str(content);
            segments.push(TemplateSegment::Raw({
                content.replace("\\{", "{").replace("\\}", "}")
            }));
            let variable = &template[left.0 + 1..right.0];
            prev = Some(right.0 + 1);
            segments.push(TemplateSegment::Variable(variable.to_string()));
            // let value = &self.variables(variable)?;
            // output.push_str(&value);
        }
        let ending = template[prev.unwrap_or(0)..]
            .replace("\\{", "{")
            .replace("\\}", "}");
        if ending.len() > 0 {
            segments.push(TemplateSegment::Raw(ending));
        }

        Ok(FormattingTemplate(segments))
    }

    pub fn fill(&self, resolver: &impl TemplateFormatting) -> Result<String, String> {
        let mut output = String::new();
        for segment in self.0.iter() {
            match segment {
                TemplateSegment::Raw(raw) => output.push_str(raw),
                TemplateSegment::Variable(variable) => {
                    output.push_str(&resolver.variables(variable)?)
                }
            }
        }
        Ok(output)
    }
}

// i should create a template struct so i dont have to reparse it every time
pub trait TemplateFormatting {
    fn variables(&self, variable: &str) -> Result<String, String>;
}

pub struct PassageFormatterBuilder {
    // can use book, chapter, verse, content
    pub verse: Option<String>,

    // the text that joins all verses together
    pub join_verses: Option<String>,

    // can use verses, the segment label, book
    pub segment: Option<String>,

    // the text that joins all segments together
    pub join_segments: Option<String>,

    // this is when there is a new chapter in the middle of a segment
    // can use segments, the segment label, book
    // book, chapter, start verse, end verse, verses
    pub chapter: Option<String>,

    // the text that joins all chapters together
    pub join_chapters: Option<String>,

    // can use book, segments label, segments
    pub text: Option<String>,
}

impl PassageFormatterBuilder {
    pub fn new() -> Self {
        Self {
            verse: None,
            join_verses: None,
            segment: None,
            join_segments: None,
            chapter: None,
            join_chapters: None,
            text: None,
        }
    }

    // can use book, chapter, verse, content
    pub fn verse(mut self, verse: impl Into<String>) -> Self {
        self.verse = Some(verse.into());
        self
    }

    // the text that joins all verses together
    pub fn join_verses(mut self, join_verses: impl Into<String>) -> Self {
        self.join_verses = Some(join_verses.into());
        self
    }

    // can use verses, the segment label, book
    pub fn segment(mut self, segment: impl Into<String>) -> Self {
        self.segment = Some(segment.into());
        self
    }

    // the text that joins all segments together
    pub fn join_segments(mut self, join_segments: impl Into<String>) -> Self {
        self.join_segments = Some(join_segments.into());
        self
    }

    // this is when there is a new chapter in the middle of a segment
    // can use segments, the segment label, book
    // book, chapter, start verse, end verse, verses
    pub fn chapter(mut self, chapter: impl Into<String>) -> Self {
        self.chapter = Some(chapter.into());
        self
    }

    // the text that joins all chapters together
    pub fn join_chapters(mut self, join_chapters: impl Into<String>) -> Self {
        self.join_chapters = Some(join_chapters.into());
        self
    }

    // can use book, segments label, segments
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn build(self) -> PassageFormatter {
        PassageFormatter {
            verse: self
                .verse
                .unwrap_or_else(|| String::from("[{chapter}:{verse}] {content}")),
            join_verses: self.join_verses.unwrap_or_else(|| String::from("\n")),
            segment: self.segment.unwrap_or_else(|| String::from("{verses}")),
            join_segments: self.join_segments.unwrap_or_else(|| String::from("\n\n")),
            chapter: self.chapter.unwrap_or_else(|| String::from("{verses}")),
            join_chapters: self.join_chapters.unwrap_or_else(|| String::from("\n")),
            text: self
                .text
                .unwrap_or_else(|| String::from("### {book} {label}\n\n{segments}")),
        }
    }
}

pub struct PassageFormatter {
    // can use book, chapter, verse, content
    pub verse: String,

    // the text that joins all verses together
    pub join_verses: String,

    // can use verses, the segment label, book
    pub segment: String,

    // the text that joins all segments together
    pub join_segments: String,

    // this is when there is a new chapter in the middle of a segment
    // can use segments, the segment label, book
    // book, chapter, start verse, end verse, verses
    pub chapter: String,

    // the text that joins all chapters together
    pub join_chapters: String,

    // can use book, segments label, segments
    pub text: String,
}

pub struct VerseFormatParameters<'a> {
    pub book: &'a str,
    pub chapter: usize,
    pub verse: usize,
    pub content: &'a str,
}

impl<'a> TemplateFormatting for VerseFormatParameters<'a> {
    fn variables(&self, variable: &str) -> Result<String, String> {
        Ok(match variable {
            "book" => self.book.to_string(),
            "chapter" => self.chapter.to_string(),
            "verse" => self.verse.to_string(),
            "content" => self.content.to_string(),
            _ => Err(format!(
                "'{}' is not a valid template identifier.",
                variable
            ))?,
        })
    }
}

pub struct SegmentFormatParameters<'a> {
    pub book: &'a str,
    pub label: &'a str,
    pub verses: &'a str,
}

impl<'a> TemplateFormatting for SegmentFormatParameters<'a> {
    fn variables(&self, variable: &str) -> Result<String, String> {
        Ok(match variable {
            "book" => self.book.to_string(),
            "label" => self.label.to_string(),
            "verses" => self.verses.to_string(),
            _ => Err(format!(
                "'{}' is not a valid template identifier.",
                variable
            ))?,
        })
    }
}

pub struct ChapterFormatParameters<'a> {
    pub book: &'a str,
    pub chapter: usize,
    pub start_verse: usize,
    pub end_verse: usize,
    pub verses: &'a str,
}

impl<'a> TemplateFormatting for ChapterFormatParameters<'a> {
    fn variables(&self, variable: &str) -> Result<String, String> {
        Ok(match variable {
            "book" => self.book.to_string(),
            "chapter" => self.chapter.to_string(),
            "start_verse" => self.start_verse.to_string(),
            "end_verse" => self.end_verse.to_string(),
            "verses" => self.verses.to_string(),
            _ => Err(format!(
                "'{}' is not a valid template identifier.",
                variable
            ))?,
        })
    }
}

pub struct PassageFormatParameters<'a> {
    pub book: &'a str,
    pub segments: &'a str,
    pub label: &'a str,
}

impl<'a> TemplateFormatting for PassageFormatParameters<'a> {
    fn variables(&self, variable: &str) -> Result<String, String> {
        Ok(match variable {
            "book" => self.book.to_string(),
            "label" => self.label.to_string(),
            "segments" => self.segments.to_string(),
            _ => Err(format!(
                "'{}' is not a valid template identifier.",
                variable
            ))?,
        })
    }
}
