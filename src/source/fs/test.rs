use futures::Stream;
use std::{
    env,
    fs::{self, File},
};

use crate::{
    config::FsLogSourceConfig,
    source::{LogRecord, LogSource},
};

use super::FsLogSource;

fn record(title: &str, body: &str) -> LogRecord {
    LogRecord {
        title: String::from(title),
        body: String::from(body),
    }
}

#[test]
fn test() {
    let base_path = env::temp_dir().join("logram_test");
    if base_path.exists() {
        fs::remove_dir_all(&base_path).unwrap();
    }
    fs::create_dir(&base_path).unwrap();

    let base_path_string = base_path.display().to_string();
    let config = FsLogSourceConfig {
        entries: vec![base_path_string],
    };
    let source = FsLogSource::new(config).unwrap();
    let mut stream = source.into_stream().wait();

    let file_path = base_path.join("file");
    let new_file_path = base_path.join("file_renamed");

    File::create(&file_path).unwrap();
    assert_eq!(
        stream.next().unwrap().unwrap(),
        record("Filesystem", "/tmp/logram_test/file created")
    );

    fs::write(&file_path, b"content").unwrap();
    assert_eq!(
        stream.next().unwrap().unwrap(),
        record("/tmp/logram_test/file", "content")
    );

    fs::rename(&file_path, &new_file_path).unwrap();
    assert_eq!(
        stream.next().unwrap().unwrap(),
        record(
            "Filesystem",
            "/tmp/logram_test/file renamed to /tmp/logram_test/file_renamed",
        )
    );

    fs::remove_file(&new_file_path).unwrap();
    assert_eq!(
        stream.next().unwrap().unwrap(),
        record("Filesystem", "/tmp/logram_test/file_renamed removed")
    );

    fs::remove_dir(&base_path).unwrap();
    assert_eq!(
        stream.next().unwrap().unwrap(),
        record("Filesystem", "/tmp/logram_test removed")
    );
}
