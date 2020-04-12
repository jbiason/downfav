use std::convert::From;
use std::fs::File;
use std::path::Path;
use std::time::Duration;

#[derive(Debug)]
pub struct Attachment {
    url: String,
}

impl From<&elefren::entities::attachment::Attachment> for Attachment {
    fn from(origin: &elefren::entities::attachment::Attachment) -> Self {
        println!("Found attachment: {}", origin.url);
        Self {
            url: origin.url.to_string(),
        }
    }
}

impl Attachment {
    pub fn get_filename(&self) -> String {
        let mut frags = self.url.rsplitn(2, '/');

        if let Some(path_part) = frags.next() {
            dbg!(path_part.split('?').next().unwrap_or(&self.url).to_string())
        } else {
            // this is, most of the time, bad (due special characters -- like '?' -- and path)
            dbg!(self.url.to_string())
        }
    }

    pub fn download(&self, local_filename: &Path) {
        let mut fp = File::create(local_filename).expect("Failed to create file");
        reqwest::Client::builder()
            .timeout(Duration::from_secs(600))
            .build()
            .unwrap()
            .get(&self.url)
            .send()
            .unwrap()
            .copy_to(&mut fp)
            .unwrap();
    }
}
