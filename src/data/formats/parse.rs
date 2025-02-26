use std::path::Path;

use crate::data::bible_data::BibleData;

pub trait ParseBibleData: Sized {
    fn parse_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>>;
    fn as_bible_data(self) -> Result<BibleData, Box<dyn std::error::Error>>;
}
