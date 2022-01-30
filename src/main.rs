extern crate feed;
extern crate rss;

use crate::item::clone_item;
use crate::item::download_item;


mod item;

use std::{
    env,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    time::Duration,
};

use feed::ChannelGetters;
use feed::ItemGetters;

use rss::{Channel, Item};

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
    let root_path_name = map_feed_title_to_dirname(channel.title());
    path.push(&feed_root);
    path.push(&root_path_name);
    fs::create_dir_all(&path).unwrap();

    let path_buf_for_rssxml = path.clone().join("rss.xml");
    let mut rssxml_file = File::create(path_buf_for_rssxml).unwrap();

    let mut cloned_channel = channel.clone();
    let map = cloned_channel
        .items()
        .iter()
        .take(num)
        .map(|item| {
            clone_item(
                &map_feed_title_to_dirname(cloned_channel.title()),
                item,
                &my_url,
            )
        })
        .collect::<Vec<Item>>();

    cloned_channel.set_items(map);

    rssxml_file
        .write_all(cloned_channel.to_string().as_bytes())
        .unwrap();

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

fn map_feed_title_to_dirname(title: &str) -> String {
    return title.replace(' ', "_").to_owned(); // TODO use extension from item
}
