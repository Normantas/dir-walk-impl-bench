use futures_lite::future::block_on;
use futures_lite::stream::StreamExt;

pub fn walkdir(root_dir: String) {
    let walker = walkdir::WalkDir::new(root_dir);
    for _entry in walker.into_iter().filter_map(|e| e.ok()) {
        
    }
}

pub fn async_walkdir(root_dir: String) {
    block_on(async {
        let mut entries = async_walkdir::WalkDir::new(root_dir);
        loop {
            match entries.next().await {
                Some(Ok(_entry)) => (),
                Some(Err(_)) => {
                    break;
                },
                None => break,
            }
        }
    });
}

pub fn fts(root_dir: String) {
    let walker = fts::walkdir::WalkDir::new(fts::walkdir::WalkDirConf::new(root_dir));
    for _entry in walker.into_iter().filter_map(|e| e.ok()) {
        
    }
}

pub fn jwalk(root_dir: String) {
    let walker = jwalk::WalkDir::new(root_dir);
    for _entry in walker.into_iter().filter_map(|e| e.ok()) {
        
    }
}