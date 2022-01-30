# rss_downloader

Snapshots and downloads a media (mp3 at moment) rss feed in a format that can be served on your own server.

Using it for storing Patreon content I subscribe to.

## Run
```sh
FEED_ROOT=##### MY_URL=##### FEED_URL=##### NUM=100 cargo run --release
```

**NUM** how many of the items to download. Using same items order as original feed xml
**FEED_ROOT** absolute location feed should be downloaded to
**MY_URL** url where you will serve the downloaded content from
**FEED_URL** rss feed url to download (i.e. patreon url)

## Result

feed root path _(from FEED_ROOT)_ `/` `feedtitle` _(with underscores)_ `/` `media1.mp3`, `media2.mp3` ... `median.mp3`, `rss.xml`
