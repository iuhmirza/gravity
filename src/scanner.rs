use std::{path::{PathBuf}, pin::Pin, sync::Arc, cmp};

use tokio::{fs, sync::Mutex, task, time};

use crate::{collector::VecCollector, entry::{Entry, EntryKind}};
use crate::collector::Collector;

pub async fn scan(path: PathBuf) -> Result<Vec<Entry>, std::io::Error> {
    let collector = Arc::new(Mutex::new(VecCollector::new()));
    println!("Scanning {}", path.display());
    let start = time::Instant::now();
    scan_dir(path, collector.clone()).await?;
    let end = time::Instant::now();
    println!("Done! Async scan took: {:?}", (end-start));
    Ok(collector.lock().await.collect())
}

pub fn scan_sync(path: PathBuf) -> Result<Vec<Entry>, std::io::Error> {
    let mut collector = VecCollector::new();
    println!("Scanning {}", path.display());
    let start = time::Instant::now();
    scan_dir_sync(path, &mut collector)?;
    let end = time::Instant::now();
    println!("Done! Sync scan took: {:?}", (end-start));
    Ok(collector.collect())
}

type ScanResult = Pin<Box<dyn Future<Output = Result<u64, std::io::Error>> + Send>>;

fn scan_dir(path: PathBuf, collector: Arc<Mutex<impl Collector + Send + 'static>>) -> ScanResult {
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
                    collector.insert(entry);
                }
            } else {
                continue;
            }
        }
        while let Some(result) = set.join_next().await {
            let size = result??;
            total_size += size;
            largest_element = cmp::max(largest_element, size);
        }
        let entry = Entry::new(path, EntryKind::Directory, total_size, total_size-largest_element);
        {
            let mut collector = collector.lock().await;
            collector.insert(entry);
        }
        Ok(total_size)
    })
}

fn scan_dir_sync(path: PathBuf, collector: &mut impl Collector) -> Result<u64, std::io::Error> {
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
            collector.insert(entry);
            max_element_size = cmp::max(max_element_size, metadata.len());
            total_size += metadata.len();
        } else if path.is_dir() {
            let size = scan_dir_sync(path, collector)?;
            max_element_size = cmp::max(max_element_size, size);
            total_size += size;
        }
    }
    let entry = Entry::new(path, EntryKind::Directory, total_size, total_size-max_element_size);
    collector.insert(entry);
    Ok(total_size)
}

#[cfg(test)]
mod tests {
    use crate::collector::{HeapCollector, VecCollector};

    use super::*;
    #[tokio::test]
    async fn test_sync_vs_async() {
        let path = std::path::Path::new("/home").to_path_buf();
        
        let mut collector1 = VecCollector::new();
        let start1 = std::time::Instant::now();
        scan_dir_sync(path.clone(), &mut collector1).unwrap();
        let end1 = std::time::Instant::now();
        
        let collector2 = Arc::new(Mutex::new(VecCollector::new()));
        let start2 = std::time::Instant::now();
        scan_dir(path, collector2.clone()).await.unwrap();
        let end2 = std::time::Instant::now();

        println!("sync: {:?}, async: {:?}", end1-start1, end2-start2);

        assert_eq!(collector1.collect(), collector2.lock().await.collect());
    }
    
    #[tokio::test]
    async fn test_vec_vs_heap() {
        let path = std::path::Path::new("/home").to_path_buf();
        
        let mut collector1 = VecCollector::new();
        let start1 = std::time::Instant::now();
        scan_dir_sync(path.clone(), &mut collector1).unwrap();
        let end1 = std::time::Instant::now();
        
        let mut collector2 = HeapCollector::new();
        let start2 = std::time::Instant::now();
        scan_dir_sync(path.clone(), &mut collector2).unwrap();
        let end2 = std::time::Instant::now();

        println!("vec: {:?}, heap: {:?}", end1-start1, end2-start2);
        let vec1 = collector1.collect();
        let vec2 = collector2.collect();
        for (e1, e2) in std::iter::zip(vec1, vec2) {
            assert_eq!(e1, e2);
        }
    }
}