use elefren::entities::status::Status;
use html2md;
use std::convert::From;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use crate::storage::attachment::Attachment;
use crate::storage::storage::Storage;

#[derive(Debug)]
pub struct Filesystem {
    id: String,
    account: String,
    text: String,
    attachments: Vec<Attachment>,
}

impl Filesystem {
    /// The directory in which the data from this toot will be saved.
    fn dir(&self) -> PathBuf {
        Path::new("data").join(&self.account).join(&self.id)
    }

    /// Make sure the path structure exists for saving the data.
    fn create_dirs(&self) {
        std::fs::create_dir_all(self.dir()).expect("Failed to create storage directory");
    }

    /// Save the content in the directory.
    fn save_content(&self) {
        let filename = self.dir().join("toot.md");
        let mut fp = File::create(filename).expect("Failed to create file");
        fp.write_all(self.text.as_bytes())
            .expect("Failed to save content");
    }

    /// Save the attachments.
    fn save_attachments(&self) {
        self.attachments.iter().for_each(|attachment| {
            let filename = self.dir().join(attachment.get_filename());
            attachment.download(filename.as_path());
        })
    }
}

impl Storage for Filesystem {
    fn open(&self) {
        dbg!(self.create_dirs());
    }

    fn get_id(&self) -> &String {
        &self.id
    }

    fn save(&self) {
        self.save_content();
        self.save_attachments();
    }
}

impl From<&Status> for Filesystem {
    fn from(origin: &Status) -> Self {
        println!("Downloading ID: {}", origin.id);

        Self {
            id: origin.id.to_string(),
            account: origin.account.acct.to_string(),
            text: html2md::parse_html(&origin.content),
            // on save, we download those URLs
            attachments: origin
                .media_attachments
                .iter()
                .map(|attachment| Attachment::from(attachment))
                .collect(),
        }
    }
}
