extern crate feed;
extern crate rss;
use std::{
    env,
    fmt::format,
    fs::{self, File},
    io::{copy, Cursor, Write},
    path::{Path, PathBuf},
    time::Duration,
};

use feed::{ChannelGetters, EnclosureGetters, FromUrl, ItemGetters};
use reqwest::{
    blocking::Client,
    blocking::Response,
    blocking::{self, Request, RequestBuilder},
};
use rss::{Channel, Item, EnclosureBuilder, Enclosure};
use std::convert::TryInto;

fn main() {
    let feed_url = match env::var("FEED_URL") {
        Ok(x) => x,
        Err(_x) => panic!("FEED_URL not defined."),
    };

    let my_url = match env::var("MY_URL") {
        Ok(x) => x,
        Err(_x) => panic!("MY_URL not defined."),
    };

    let num = match env::var("NUM") {
        Ok(x) => x.parse::<usize>().unwrap(),
        Err(_x) => panic!("NUM not defined."),
    };

    let feed_root = match env::var("FEED_ROOT") {
        Ok(x) => x,
        Err(_x) => panic!("FEED_ROOT not defined."),
    };

    let source_rss_xml_content = reqwest::blocking::get(&feed_url).unwrap().bytes().unwrap();

    let channel = Channel::read_from(&source_rss_xml_content[..]).unwrap();
    let mut path = PathBuf::new();
    let root_path_name = map_feed_title_to_filename(channel.title());
    path.push(&feed_root);
    path.push(&root_path_name);
    fs::create_dir_all(&path).unwrap();

    let path_buf_for_rssxml = path.clone().join("rss.xml");
    let mut rssxml_file = File::create(path_buf_for_rssxml).unwrap();

    let mut cloned_channel = channel.clone();
    let map = cloned_channel.items().iter().take(num)
    .map(|item| {
        let mut item_cloned = item.clone();
        let mut enclosure = Enclosure::default();
        enclosure.set_url(format!(
            "{}/{}/{}",
            &my_url,
            map_feed_title_to_filename(&cloned_channel.title()),
            map_item_title_to_filename(&item.title().unwrap()).as_str()
        ));
        enclosure.set_mime_type(item.enclosure().unwrap().mime_type());
        enclosure.set_length(item.enclosure().unwrap().length());
        item_cloned.set_enclosure(enclosure);
        item_cloned
    })
    .collect::<Vec<Item>>();

    cloned_channel.set_items(map);

    rssxml_file.write_all(cloned_channel.to_string().as_bytes()).unwrap();

    let reqwest_client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(100)) // we needed to increase the timeout
        .build()
        .unwrap();

    let item_count = channel.items().len();
    println!("Number of items:{}", item_count);
    println!();

    channel
        .items()
        .iter()
        .take(num)
        .for_each(|item| download_item(item, &path, &reqwest_client));
}

fn download_item(item: &Item, path_buf: &PathBuf, reqwest_client: &Client) {
    let filename = map_item_title_to_filename(item.title().unwrap());
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
                .get(item.enclosure().unwrap().url())
                .build()
                .unwrap(),
        )
        .unwrap();
    let mut item_content_cursor = Cursor::new(item_content_response.bytes().unwrap());

    println!("Copying... ");
    let mut file = File::create(file_path_buf).unwrap();
    copy(&mut item_content_cursor, &mut file).unwrap();
}

fn map_item_title_to_filename(title: &str) -> String {
    return title.replace('/', "") + &".mp3".to_owned(); // TODO use extension from item
}

fn map_feed_title_to_filename(title: &str) -> String {
    return title.replace(' ', "_").to_owned(); // TODO use extension from item
}