use std::borrow::Cow;
use std::io;
use std::path::{Path, PathBuf};

use rand::{self, Rng};
use rocket::request::FromParam;
use rocket::tokio::fs;

#[derive(UriDisplayPath)]
pub struct PasteId<'a>(Cow<'a, str>);

pub const ROOT: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "\\", "upload");

impl PasteId<'_> {
    pub fn new(size: usize) -> PasteId<'static> {
        const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
        let mut id = String::with_capacity(size);
        let mut rng = rand::thread_rng();
        
        for _ in 0..size {
            id.push(BASE62[rng.gen::<usize>() % 62] as char);
        }
        
        PasteId(Cow::Owned(id))
    }
    
    pub fn file_path(&self) -> PathBuf {
        Path::new(ROOT).join(self.0.as_ref())
    }
    
    pub fn get_upload_dir() -> &'static Path {
        Path::new(ROOT)
    }
    
    pub async fn get_all_files() -> Result<Vec<String>, io::Error> {
        let mut files: Vec<String> = Vec::new();
        let mut entries = fs::read_dir(PasteId::get_upload_dir()).await?;
        println!("{:?}", PasteId::get_upload_dir());
        while let entry = entries.next_entry().await? {
            match entry {
                Some(e) => {
                    println!("{:?}",e);
                    files.push(e.file_name().into_string().unwrap());
                },
                None => { break }
            }
        }
        Ok(files)
    }
    
}

impl<'a> FromParam<'a> for PasteId<'a> {
    type Error = &'a str;
    
    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        param.chars().all(|c| c.is_ascii_alphanumeric())
             .then(|| PasteId(param.into()))
             .ok_or(param)
    }
}