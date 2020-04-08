use chrono::prelude::*;

use log::info;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::fmt::Display;
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn run() {
    let (tx, rx) = channel();

    let mut watcher_ = watcher(tx, Duration::from_millis(100)).unwrap();

    watcher_.watch("static", RecursiveMode::Recursive).unwrap();
    loop {
        match rx.recv() {
            Ok(event) => match_enent(event),
            Err(e) => info!("watch error: {:?}", e),
        }
    }
}

fn match_enent(event: DebouncedEvent) {
    use DebouncedEvent::{
        Chmod, Create, Error, NoticeRemove, NoticeWrite, Remove, Rename, Rescan, Write,
    };
    match event {
        NoticeWrite(path) => info!("NoticeWrite path [ {} ]", path.to_str().unwrap()),
        NoticeRemove(path) => info!("NoticeRemove path [ {} ]", path.to_str().unwrap()),
        Create(path) => info!("Create path [ {} ]", path.to_str().unwrap()),
        Write(path) => info!("Write path [ {} ]", path.to_str().unwrap()),
        Chmod(path) => info!("Chmod path [ {} ]", path.to_str().unwrap()),
        Remove(path) => info!("Remove path [ {} ]", path.to_str().unwrap()),
        Rename(path_from, path_to) => info!(
            "Rename from path [ {} ] to path[ {} ]",
            path_from.to_str().unwrap(),
            path_to.to_str().unwrap()
        ),
        Rescan => info!("Rescaning"),
        Error(e, o) => info!("{}, {:?}", e, o),
    }
}
