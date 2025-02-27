use std::{path::Path, time::Instant};

use api::{bible_api::BibleAPI, passage::formatting_template::PassageFormatterBuilder};
use bible_data::{
    bible_data::BibleData,
    formats::{json::JSONBible, parse::ParseBibleData},
};

pub mod api;
pub mod bible_data;
pub mod related_media;

fn main() {
    let path = Path::new("/home/dglinuxtemple/esv.json");
    let data = JSONBible::parse_file(path)
        .unwrap()
        .as_bible_data()
        .unwrap();
    let api = BibleAPI::load(data);

    // // 13ms on my machine with --release
    // let start = Instant::now();
    // for _ in 1..10_000 {
    //     let passage = api.parse_reference("Ephesians 1:1-2,4-6,22-2:2,5,3:21-4:2");
    // }
    // println!("{}ms", start.elapsed().as_millis());

    let passage = api.parse_reference("Ephesians 1:1-2,4-6,22-2:2,5,3:21-4:2");
    if let Some(passage) = passage {
        for verse in passage.clone().into_iter() {
            println!(
                "[{} {}:{}] {:#?}",
                verse.get_book().get_name(),
                verse.chapter_number(),
                verse.verse_number(),
                // verse.get_related_media()
                verse.get_content().unwrap_or("")
            );
        }

        let formatter = PassageFormatterBuilder::new().build();
        let output = passage.format(&formatter);
        println!("{}", output);
    } else {
        println!("No passage found");
    }
}
