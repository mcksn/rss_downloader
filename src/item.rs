extern crate urlencoding;

use std::{
    fs::{File},
    io::{copy, Cursor},
    path::{PathBuf},
};

use feed::{ChannelGetters, EnclosureGetters, ItemGetters};
use reqwest::{
    blocking::Client,
    blocking::{self},
};
use rss::{Enclosure, Item};

use urlencoding::encode;

pub(crate) trait Cloneable {
    fn clone_item(&self, path_to_item: &str, my_url: &str) -> Self;
}

impl Cloneable for Item {

    fn clone_item(&self, path_to_item: &str, my_url: &str) -> Item {
        let mut item_cloned = self.clone();
        let mut enclosure = Enclosure::default();
        enclosure.set_url(&format!(
            "{}/{}/{}",
            &my_url,
            encode(path_to_item),
            encode(map_item_title_to_filename(&self.title().unwrap()).as_str())
        ));
        enclosure.set_mime_type(self.enclosure().unwrap().mime_type());
        enclosure.set_length(self.enclosure().unwrap().length());
        item_cloned.set_enclosure(enclosure);
        item_cloned
    }
    
}

pub(crate) trait Downloadable {
    fn download(&self, path_buf: &PathBuf, reqwest_client: &Client);
}

impl Downloadable for Item {

    fn download(&self, path_buf: &PathBuf, reqwest_client: &Client) {
    let filename = map_item_title_to_filename(self.title().unwrap());
    println!("Downloading... {:?}", &filename);
    let file_path_buf = path_buf.join(&filename);
    println!("Filename length:{}", &filename.len());
    if file_path_buf.is_file() {
        println!("Skiping...");
        return;
    }
    let item_content_response: blocking::Response = reqwest_client
        .execute(
            reqwest_client
                .get(self.enclosure().unwrap().url())
                .build()
                .unwrap(),
        )
        .unwrap();
    let mut item_content_cursor = Cursor::new(item_content_response.bytes().unwrap());

    println!("Copying... ");
    let mut file = File::create(file_path_buf).unwrap();
    copy(&mut item_content_cursor, &mut file).unwrap();
}

}

fn map_item_title_to_filename(title: &str) -> String {
    return title.replace('/', "") + &".mp3".to_owned(); // TODO use extension from item
}
