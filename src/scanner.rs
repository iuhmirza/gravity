use std::{collections::BinaryHeap, path::PathBuf, pin::Pin, sync::Arc};

use tokio::{fs, sync::Mutex, task};

use crate::entry::{Entry, EntryKind};

type Collector = Arc<Mutex<BinaryHeap<Entry>>>;

pub async fn scan(path: PathBuf, collector: Collector) -> Result<(), std::io::Error> {
    scan_dir(path, collector.clone()).await?;
    Ok(())
}

type ScanResult = Pin<Box<dyn Future<Output = Result<u64, std::io::Error>> + Send>>;

fn scan_dir(path: PathBuf, collector: Collector) -> ScanResult {
    Box::pin(async move {
        let mut dir = fs::read_dir(&path).await?;
        let mut set = task::JoinSet::new();
        let mut total_size = 0;
        let mut largest_element = 0;
        while let Some(entry) = dir.next_entry().await? {
            let entry = entry.path();
            let metadata = fs::symlink_metadata(&entry).await?;
            if metadata.is_symlink() {
                continue;
            } else if metadata.is_dir() {
                set.spawn(scan_dir(entry, collector.clone()));
            } else if metadata.is_file() {
                let filesize = metadata.len();
                let entry = Entry::new(entry, EntryKind::File, filesize, filesize);
                total_size += entry.size();
                largest_element = std::cmp::max(entry.size(), largest_element);
                {
                    let mut collector = collector.lock().await;
                    collector.push(entry);
                }
            } else {
                continue;
            }
        }
        while let Some(result) = set.join_next().await {
            let size = result??;
            total_size += size;
            largest_element = std::cmp::max(largest_element, size);
        }
        let entry = Entry::new(path, EntryKind::Directory, total_size, total_size-largest_element);
        {
            let mut collector = collector.lock().await;
            collector.push(entry);
        }
        Ok(total_size)
    })
}