use std::path::Path;

use api::bible_api::BibleAPI;
use data::{
    bible_data::BibleData,
    formats::{json::JSONBible, parse::ParseBibleData},
};

pub mod api;
pub mod data;

fn main() {
    let path = Path::new("/home/dglinuxtemple/esv.json");
    let data = JSONBible::parse_file(path)
        .unwrap()
        .as_bible_data()
        .unwrap();
    let api = BibleAPI::load(data);
    let passage = api.parse_reference("Ephesians 1:1-2,4-6,22-2:2,5,3:21-4:2");
    if let Some(passage) = passage {
        for seg in passage.into_iter() {
            println!(
                "[{}:{}] {}",
                seg.chapter_number(),
                seg.verse_number(),
                seg.get_content().unwrap_or("")
            );
        }
    } else {
        println!("No passage found");
    }
}
