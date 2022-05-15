use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
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

    // Add a path to be watched.
    // All files and directories at that path and below will be monitored for changes.
    match watcher.watch(path, RecursiveMode::Recursive) {
        Ok(()) => {}
        Err(_) => return,
    };

    loop {
        match rx.recv() {
            Ok(DebouncedEvent::NoticeRemove(_)) => return,
            Err(_) => return,
            Ok(_) => {} // Any other event than 'delete' is ignored
        }
    }
}
