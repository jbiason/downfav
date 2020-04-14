use elefren::entities::status::Status;
use html2md;
use std::convert::From;

use crate::storage::attachment::Attachment;
use crate::storage::storage::Storage;

/// Our data content.
#[derive(Debug)]
pub struct Data {
    pub id: String,
    pub account: String,
    pub text: String,
    pub attachments: Vec<Attachment>,
}

/// Convert the incoming Status from Elefren to ours.
impl From<&Status> for Data {
    fn from(origin: &Status) -> Self {
        println!("Downloading ID: {}", origin.id);

        Self {
            id: origin.id.to_string(),
            account: origin.account.acct.to_string(),
            text: build_text(origin),
            attachments: origin
                .media_attachments
                .iter()
                .map(|attachment| Attachment::from(attachment))
                .collect(),
        }
    }
}

impl Data {
    pub fn save<T: Storage>(&self, storage: &T) {
        storage.save(self);
    }
}

fn build_text(status: &Status) -> String {
    let base_content = html2md::parse_html(&status.content);
    let source = &status.url;
    let title = &status.spoiler_text;

    let mut result = String::new();
    if title.len() > 0 {
        result.push_str(title);
        result.push_str("\n");
    }

    result.push_str(&html2md::parse_html(&base_content));

    if let Some(url) = source {
        result.push_str("\n");
        result.push_str(&url);
    }

    result
}
