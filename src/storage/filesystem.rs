use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use crate::storage::data::Data;
use crate::storage::storage::Storage;

pub struct Filesystem {}

impl Storage for Filesystem {
    fn save(&self, data: &Data) {
        self.create_dirs(data);
        self.save_content(data);
        self.save_attachments(data);
    }
}

impl Filesystem {
    pub fn new() -> Self {
        Self {}
    }

    /// The directory in which the data from this toot will be saved.
    fn dir(&self, data: &Data) -> PathBuf {
        Path::new("data").join(&data.account).join(&data.id)
    }

    /// Make sure the path structure exists for saving the data.
    fn create_dirs(&self, data: &Data) {
        std::fs::create_dir_all(self.dir(data)).expect("Failed to create storage directory");
    }

    /// Save the content in the directory.
    fn save_content(&self, data: &Data) {
        let filename = self.dir(data).join("toot.md");
        let mut fp = File::create(filename).expect("Failed to create file");
        fp.write_all(data.text.as_bytes())
            .expect("Failed to save content");
    }

    /// Save the attachments.
    fn save_attachments(&self, data: &Data) {
        data.attachments.iter().for_each(|attachment| {
            let filename = self.dir(data).join(attachment.get_filename());
            attachment.download(filename.as_path());
        })
    }
}
