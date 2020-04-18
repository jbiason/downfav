use std::collections::HashMap;
use std::path::Path;

use reqwest::multipart::Form;
use reqwest::multipart::Part;
use reqwest::Error;
use serde_derive::Deserialize;

use crate::config::JoplinConfig;
use crate::storage::data::Data;
use crate::storage::storage::Storage;

static INLINABLE: [&'static str; 4] = ["jpeg", "jpg", "png", "gif"];

/// This is the folder structured returned by Joplin. It is here so Reqwests can
/// unjson the data (there are more fields, but these are the only ones we need
/// right now).
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Folder {
    id: String,
    title: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Resource {
    id: String,
    filename: String,
}

/// Connection to Joplin.
pub struct Joplin {
    port: u32,
    token: String,
    folder_id: String,
    client: reqwest::Client,
}

impl Storage for Joplin {
    fn save(&self, record: &Data) {
        let resources = dbg!(self.save_attachments(&record));
        let mut text = record.text.to_string();
        let title = format!("{}/{}", record.account, record.id);
        Joplin::add_resources_to_text(&mut text, &resources);
        dbg!(self.save_content(&title, &text, &record.source));
    }
}

impl Joplin {
    pub fn new_from_config(config: &JoplinConfig) -> Joplin {
        if let Some(folder_id) = Joplin::find_folder(config) {
            Joplin {
                port: config.port,
                token: config.token.to_string(),
                folder_id: folder_id,
                client: reqwest::Client::new(),
            }
        } else {
            println!("The notebook {} does not exist", &config.folder);
            panic!("The specified notebook does not exist");
        }
    }

    fn find_folder(config: &JoplinConfig) -> Option<String> {
        if let Ok(folders) = dbg!(Joplin::get_folder_list(config)) {
            for folder in folders {
                if folder.title == *config.folder {
                    return Some(folder.id);
                }
            }
            None
        } else {
            println!("Failed to retrieve the notebook list");
            panic!("Failed to retrieve Joplin notebook list");
        }
    }

    fn get_folder_list(config: &JoplinConfig) -> Result<Vec<Folder>, Error> {
        let base_url = format!(
            "http://localhost:{port}/folders?token={token}",
            port = config.port,
            token = config.token
        );
        let folders: Vec<Folder> = reqwest::get(&base_url)?.json()?;
        Ok(folders)
    }

    fn add_resources_to_text(text: &mut String, resources: &Vec<Resource>) {
        resources.iter().for_each(|resource| {
            let link = format!(
                "{inline}[{filename}](:/{resource})",
                inline = if Joplin::is_inlineable(&resource.filename) {
                    "!"
                } else {
                    ""
                },
                filename = resource.filename,
                resource = resource.id
            );
            text.push_str("\n\n");
            text.push_str(&link);
        });
    }

    fn is_inlineable(filename: &String) -> bool {
        if let Some(extension) = Path::new(filename).extension() {
            INLINABLE
                .iter()
                .any(|ext| *ext == extension.to_str().unwrap_or(""))
        } else {
            false
        }
    }

    fn save_attachments(&self, record: &Data) -> Vec<Resource> {
        record
            .attachments
            .iter()
            .map(|attachment| {
                let mut buffer: Vec<u8> = vec![];
                attachment.download().copy_to(&mut buffer).unwrap();
                let resource_id =
                    dbg!(self.upload_resource(attachment.filename().to_string(), buffer));

                Resource {
                    id: resource_id,
                    filename: attachment.filename().to_string(),
                }
            })
            .collect()
    }

    fn base_url(&self, resource: &str) -> String {
        format!(
            "http://localhost:{port}/{resource}?token={token}",
            port = self.port,
            token = self.token,
            resource = resource
        )
    }

    fn upload_resource(&self, filename: String, content: Vec<u8>) -> String {
        let props = format!("{{\"title\": \"{filename}\"}}", filename = &filename);
        let data_part = Part::bytes(content).file_name(filename);
        let props_part = Part::text(props);
        let form = Form::new()
            .part("data", data_part)
            .part("props", props_part);
        let resource: Resource = self
            .client
            .post(&self.base_url("resources"))
            .multipart(form)
            .send()
            .unwrap()
            .json()
            .unwrap();
        resource.id
    }

    fn save_content(&self, title: &String, text: &String, source: &String) {
        let mut request = HashMap::new();
        request.insert("parent_id", &self.folder_id);
        request.insert("title", &title);
        request.insert("body", &text);
        request.insert("source_url", &source);

        self.client
            .post(&self.base_url("notes"))
            .json(&request)
            .send()
            .unwrap();
    }
}
