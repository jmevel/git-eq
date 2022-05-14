extern crate notify;

use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

/// This function will wait until it receives an event that the file is deleted
/// If any error occurs during the watch process, the function will return immediately
pub fn wait_until_deleted(path: &Path) {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    //watcher.watch(path, RecursiveMode::Recursive).unwrap();
    let watch_success = match watcher.watch(path, RecursiveMode::Recursive) {
        Ok(()) => true,
        Err(_) => return
    };

    if watch_success {
        loop {
            match rx.recv() {
                Ok(event) => {
                    if let DebouncedEvent::NoticeRemove(_) = event {
                        return;
                    }
                },
                Err(_) => return,
            }
        }
    }
}