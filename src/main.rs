// use std::collections::HashMap;
extern crate feed;
extern crate rss;

use feed::{ChannelGetters, EnclosureGetters, FromUrl, ItemGetters};
use rss::Channel;

fn main() {
    let url = "";

    let channel = Channel::from_url(url).unwrap();
    println!(
        "Feed Title: {:?}",
        channel
            .items()
            .iter()
            .filter(|i| i.title().unwrap().contains("Alien"))
            .next()
            .unwrap()
            .enclosure()
            .unwrap()
            .url()
    );
}
