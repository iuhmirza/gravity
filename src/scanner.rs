use std::{collections::BinaryHeap, path::PathBuf, pin::Pin, sync::Arc};

use tokio::{fs, sync::Mutex, task};

use crate::entry::{Entry, EntryKind};

type Collector = Arc<Mutex<BinaryHeap<Entry>>>;

pub async fn scan(path: PathBuf, collector: Collector) -> Result<(), std::io::Error> {
    // scan_dir(path, collector.clone()).await?;
    // let collector1 = &mut collector.lock().await;
    let mut collector1 = BinaryHeap::new();
    let collector2 = Arc::new(Mutex::new(BinaryHeap::new()));
    let start1 = std::time::Instant::now();
    scan_dir_sync(path.clone(), &mut collector1);
    let end1 = std::time::Instant::now();
    let start2 = std::time::Instant::now();
    scan_dir(path, collector2.clone()).await?;
    let end2 = std::time::Instant::now();
    println!("sync: {:?}, async: {:?}", end1-start1, end2-start2);
    assert_eq!(collector1.into_sorted_vec(), collector2.lock().await.clone().into_sorted_vec());
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

fn scan_dir_sync(path: PathBuf, collector: &mut BinaryHeap<Entry>) -> Result<u64, std::io::Error> {
    let mut total_size = 0;
    let mut max_element_size = 0;
    for entry in std::fs::read_dir(&path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let path = entry.path();
        if path.is_symlink() {
            continue
        } else if path.is_file() {
            let entry = Entry::new(path, EntryKind::File, metadata.len(), metadata.len());
            collector.push(entry);
            max_element_size = std::cmp::max(max_element_size, metadata.len());
            total_size += metadata.len();
        } else if path.is_dir() {
            let size = scan_dir_sync(path, collector)?;
            max_element_size = std::cmp::max(max_element_size, size);
            total_size += size;
        }
    }
    let entry = Entry::new(path, EntryKind::Directory, total_size, total_size-max_element_size);
    collector.push(entry);
    Ok(total_size)
}