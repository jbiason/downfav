/*
   DOWNFAV - Download Favourites
   Copyright (C) 2020  Julio Biason

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU Affero General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Affero General Public License for more details.

   You should have received a copy of the GNU Affero General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::collections::HashMap;
use std::path::Path;

use reqwest::multipart::Form;
use reqwest::multipart::Part;
use reqwest::Error;
use serde_derive::Deserialize;

use crate::config::JoplinConfig;
use crate::storage::data::Data;
use crate::storage::helpers::make_markdown;
use crate::storage::storage::Storage;

static INLINABLE: [&'static str; 4] = ["jpeg", "jpg", "png", "gif"];

/// This is the folder structured returned by Joplin. It is here so Reqwests can
/// unjson the data (there are more fields, but these are the only ones we need
/// right now).
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct FolderList {
    items: Vec<Folder>,
    has_more: bool,
}

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
        let resources = self.save_attachments(&record);
        log::debug!("Record attachments: {:?}", resources);
        let mut text = make_markdown(record);
        let title = format!("{}/{}", record.account, record.id);
        Joplin::add_resources_to_text(&mut text, &resources);
        self.save_content(&title, &text, &record.source);
    }
}

impl Joplin {
    pub(crate) fn new_from_config(config: &JoplinConfig) -> Joplin {
        match Joplin::find_folder(config) {
            Some(folder_id) => Joplin {
                port: config.port,
                token: config.token.to_string(),
                folder_id,
                client: reqwest::Client::new(),
            },
            None => {
                panic!("The specified notebook does not exist");
            }
        }
    }

    fn find_folder(config: &JoplinConfig) -> Option<String> {
        match Joplin::get_folder_list(config) {
            Ok(folders) => {
                for folder in folders {
                    if folder.title == *config.folder {
                        return Some(folder.id);
                    }
                }
                None
            }
            Err(_) => {
                panic!("Failed to retrieve Joplin notebook list");
            }
        }
    }

    fn get_folder_list(config: &JoplinConfig) -> Result<Vec<Folder>, Error> {
        let mut page = 1;
        let mut has_more = true;
        let mut folders: Vec<Folder> = Vec::new();

        while has_more {
            let base_url = format!(
                "http://localhost:{port}/folders?token={token}&page={page}",
                port = config.port,
                token = config.token,
                page = page
            );
            let folder_list: FolderList = reqwest::get(&base_url)?.json()?;

            folder_list.items.iter().for_each(|folder| {
                folders.push(Folder {
                    // XXX this is silly and I should change this to use an
                    // iterator over the results
                    id: folder.id.to_string(),
                    title: folder.title.to_string(),
                })
            });
            page += 1;
            has_more = folder_list.has_more;
        }

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
                let resource_id = self.upload_resource(attachment.filename().to_string(), buffer);

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
