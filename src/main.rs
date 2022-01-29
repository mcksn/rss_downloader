extern crate feed;
extern crate rss;
use std::{
    env,
    fs::{self, File},
    io::{copy, Cursor},
    path::{Path, PathBuf},
};

use feed::{ChannelGetters, EnclosureGetters, FromUrl, ItemGetters};
use rss::{Channel, Item};

fn main() {
    let feed_url = match env::var("FEED_URL") {
        Ok(x) => x,
        Err(_x) => panic!("FEED_URL not defined."),
    };

    let num = match env::var("NUM") {
        Ok(x) => x.parse::<usize>().unwrap(),
        Err(_x) => panic!("NUM not defined."),
    };

    let channel = Channel::from_url(&feed_url).unwrap();
    let mut path = PathBuf::new();
    let root_path_name = channel.title().replace(" ", "_");
    path.push(&root_path_name);
    fs::create_dir_all(Path::new(&root_path_name)).unwrap();

    let item_count = channel.items().len();
    println!("Number of items:{}", item_count);
    println!();

    channel
        .items()
        .iter()
        .take(num)
        .for_each(|item| download_item(item, &path))
}

fn download_item(item: &Item, path_buf: &PathBuf) {
    let title = item.title().unwrap().replace('/', "");
    println!("Downloading... {:?}", &title);
    let file_path_buf = path_buf.join(title.to_owned() + ".mp3"); //TODO use extension from url
    println!("Filename length:{}", &title.len());
    if file_path_buf.is_file() {
        println!("Skiping...");
        return;
    }
    let item_content_response = reqwest::blocking::get(item.enclosure().unwrap().url()).unwrap();
    let mut item_content_cursor = Cursor::new(item_content_response.bytes().unwrap());

    println!("Copying... ");
    let mut file = File::create(file_path_buf).unwrap();
    copy(&mut item_content_cursor, &mut file).unwrap();
}
