use crate::qiniu::Qiniu;
use crate::qiniu::{upload_file, UploadType};
use chrono::prelude::*;
use std::path::Path;
use std::time::Duration;

// 每天同步一次
pub fn start(url: String, qiniu: Qiniu) {
    std::thread::spawn(move || {
        let path: &Path = url.as_ref();
        let mut last_sync = Local.ymd(2000, 1, 1).and_hms_milli(0, 0, 0, 1).timestamp();
        loop {
            let now = Local::now();
            if now.timestamp() - last_sync > 24 * 60 * 60 {
                last_sync = now.timestamp();
                if let Ok(file) = std::fs::File::open(path) {
                    match upload_file(
                        file,
                        format!("db_{}{}{}.sqlite", now.year(), now.month(), now.day()),
                        &qiniu,
                        UploadType::DB,
                    ) {
                        Ok(_) => println!(
                            "sync db success, {}{}{}",
                            now.year(),
                            now.month(),
                            now.day()
                        ),
                        Err(e) => println!(
                            "sync db error, {}{}{}, '{}'",
                            now.year(),
                            now.month(),
                            now.day(),
                            e
                        ),
                    };
                }
            }

            std::thread::sleep(Duration::from_secs(60));
        }
    });
}
