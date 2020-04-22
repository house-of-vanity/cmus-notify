extern crate dbus;

use crate::dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use dbus::{arg, blocking::Connection};
use notify_rust::Notification;
use std::time::Duration;

#[derive(Debug, Default)]
pub struct TrackInfo {
    title: String,
    album: String,
    duration: String,
    artist: String,
    genre: String,
}

fn format_time(sec: i64) -> String {
    let minutes = sec / 60;
    let secondes = sec % 60;
    let result = format!("{:02}:{:02}", minutes, secondes);
    result.to_string()
}

fn not_main() -> TrackInfo {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.mpris.MediaPlayer2.cmus",
        "/org/mpris/MediaPlayer2",
        Duration::from_millis(5000),
    );
    let metadata: Box<dyn arg::RefArg> = proxy
        .get("org.mpris.MediaPlayer2.Player", "Metadata")
        .unwrap();
    let mut track_info: TrackInfo = TrackInfo::default();
    let mut iter = metadata.as_iter().unwrap();

    while let Some(key) = iter.next() {
        if key.as_str() == Some("xesam:title") {
            if let Some(title) = iter.next().unwrap().as_str() {
                track_info.title = title.to_string();
            }
        }
        if key.as_str() == Some("xesam:genre") {
            if let Some(mut artists) = iter.next().unwrap().as_iter() {
                while let Some(artist) = artists.next() {
                    if let Some(mut line) = artist.as_iter() {
                        track_info.genre = line.next().unwrap().as_str().unwrap().to_string();
                    }
                }
            }
        }
        if key.as_str() == Some("xesam:album") {
            if let Some(album) = iter.next().unwrap().as_str() {
                track_info.album = album.to_string();
            }
        }
        if key.as_str() == Some("mpris:length") {
            if let Some(length) = iter.next().unwrap().as_i64() {
                track_info.duration = format_time(length / 1000000);
            }
        }
        if key.as_str() == Some("xesam:artist") {
            if let Some(mut artists) = iter.next().unwrap().as_iter() {
                while let Some(artist) = artists.next() {
                    if let Some(mut line) = artist.as_iter() {
                        track_info.artist = line.next().unwrap().as_str().unwrap().to_string();
                    }
                }
            }
        }
    }
    track_info
}

fn main() {
    let track_info = not_main();
    Notification::new()
        .summary(&format!("{}", track_info.title,))
        .appname("Cmus")
        .body(&format!(
            "{}
             {} {}",
            track_info.artist, track_info.duration, track_info.genre
        ))
//      .action("clicked", "click here")
        .icon("music")
        .show()
//      .unwrap()
//      .wait_for_action(|action| match action {
//                                      "clicked" => println!("that was correct"),
//                                      // here "__closed" is a hard coded keyword
//                                      "__closed" => println!("the notification was closed"),
//                                      _ => ()
//                                  })
        ;
}
