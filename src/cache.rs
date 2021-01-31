use druid::image::{DynamicImage, ImageError};
use std::collections::HashMap;
use std::path::PathBuf;
use std::env::temp_dir;
use std::lazy::Lazy;
use std::sync::{Mutex, Arc};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use druid::image;
use std::fs::File;
use std::io::Write;
use std::future::Future;
use bytes::Bytes;
use std::error::Error;

#[derive(Debug, Clone)]
struct SpiderError;

impl From<ImageError> for SpiderError {
    fn from(_: ImageError) -> Self {
        unimplemented!()
    }
}


struct CacheFile {
    path: String,
    image: Option<DynamicImage>,
}

struct ImageCache {
    cache: Arc<Mutex<HashMap<&'static str, CacheFile>>>,
}

impl Default for ImageCache {
    fn default() -> Self {
        ImageCache { cache: Arc::new(Mutex::new(HashMap::new())) }
    }
}

impl ImageCache {
    fn get_temp_dir(&self) -> PathBuf {
        temp_dir().join("AmbientSpider")
    }

    fn get_image_dir(&self) -> PathBuf {
        let dir = self.get_temp_dir().join("cache").join("images");
        std::fs::create_dir_all(&dir);
        dir
    }

    async fn download_bytes(&self, url: &str) -> reqwest::Result<Bytes> {
        reqwest::get(url)
            .await?
            .bytes()
            .await
    }

    async fn download_image(&self, url: &str) -> Result<(), dyn Error> {
        let dir: PathBuf = self.get_image_dir();
        let image_bytes = self.download_bytes(&url).await?;
        let image = image::load_from_memory(&img_bytes)?;
        let mut cache = self.cache.lock()?;
        let cached: &mut CacheFile = (*cache).get_mut(url)?;
        let mut file = File::create(dir.join((*cached).path.to_string()))?;
        file.write_all(image.as_bytes())?;
        (*cached).image = Some(image);
        Ok::<(), Error>(())
    }

    pub fn get_image(&self, path: &'static str) -> Option<&CacheFile> {

        let mut cache = self.cache.lock().unwrap();
        if let file = (*cache).get(path) {
            return file;
        }
        let new_path = random_string();
        let f = CacheFile { path: new_path, image: None };
        cache.insert(path.as_ref(), f);

        if path.starts_with("http") {
            tokio::task::spawn(self.download_image(path));
        }
        Some(&f)
    }
}

pub static IMAGE_CACHE: Lazy<ImageCache> = Lazy::new(|| ImageCache::default());

fn random_string() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect()
}
