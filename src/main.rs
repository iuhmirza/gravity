use std::{collections::BinaryHeap, path::Path, sync::Arc};

use tokio::sync::Mutex;

mod scanner;
mod entry;
mod collector;

#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
    println!("Scanning...");
    let collector = Arc::new(Mutex::new(BinaryHeap::new()));
    scanner::scan(Path::new("/home").to_path_buf(), collector.clone()).await?;
    let mut collector = collector.lock().await;
    // for i in 1..=10  {
    //     let entry = collector.pop().unwrap();
    //     println!("{i}: {entry:?}")
    // }
    Ok(())
}
