use std::{path::Path};



mod scanner;
mod entry;
mod collector;

#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
    let path = Path::new("/home").to_path_buf();
    let entries = scanner::scan(path.clone()).await?;
    for (i, entry) in entries.iter().enumerate() {
        println!("{i}: {entry:?}");
    }
    let entries = scanner::scan_sync(path)?;
    for (i, entry) in entries.iter().enumerate() {
        println!("{i}: {entry:?}");
    }
    Ok(())
}
