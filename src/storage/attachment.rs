use std::convert::From;
use std::time::Duration;

use reqwest::Response;

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
    pub fn filename(&self) -> String {
        let mut frags = self.url.rsplitn(2, '/');

        if let Some(path_part) = frags.next() {
            dbg!(path_part.split('?').next().unwrap_or(&self.url).to_string())
        } else {
            // this is, most of the time, bad (due special characters -- like '?' -- and path)
            dbg!(self.url.to_string())
        }
    }

    pub fn download(&self) -> Response {
        reqwest::Client::builder()
            .timeout(Duration::from_secs(600))
            .build()
            .unwrap()
            .get(&self.url)
            .send()
            .unwrap()
    }
}
