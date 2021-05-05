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

use std::borrow::Borrow;
use std::default::Default;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;

use chrono::prelude::*;
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::Handle;
use markup5ever_rcdom::NodeData;
use markup5ever_rcdom::RcDom;

use crate::config::OrgConfig;
use crate::storage::data::Data;
use crate::storage::storage::Storage;

/// Definition for the Org storage
pub struct Org {
    /// The path where the file will be stored
    path: PathBuf,
    /// The filename for the org file
    filename: String,
    /// The date being processed, needed for the header if it is a new file
    date: String,
}

/// Simple macro to recursively walk through html5ever nodes
macro_rules! keep_going {
    ($source:ident, $target:ident) => {
        for child in $source.children.borrow().iter() {
            walk(child.borrow(), $target);
        }
    };
}

/// Walk though the html5ever nodes, producing the required string in Org
/// format.
fn walk(input: &Handle, result: &mut String) {
    match input.data {
        NodeData::Text { ref contents } => {
            let text = contents.borrow().to_string();
            result.push_str(&text);
            keep_going!(input, result);
        }
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            let tag = name.local.to_string();
            match tag.as_ref() {
                "html" | "head" | "body" => keep_going!(input, result),
                "p" | "br" => {
                    keep_going!(input, result);
                    result.push_str("\n  ");
                }
                "span" => {
                    let attrs = attrs.borrow();
                    let classes_attr = attrs
                        .iter()
                        .find(|attr| attr.name.local.to_string() == "class");
                    match classes_attr {
                        Some(classes) => {
                            if classes.value.contains("ellipsis") {
                                keep_going!(input, result);
                                result.push_str("...");
                            } else if !classes.value.contains("invisible") {
                                keep_going!(input, result);
                            }
                        }
                        None => keep_going!(input, result),
                    }
                }
                "a" => {
                    let attrs = attrs.borrow();
                    let rels = attrs
                        .iter()
                        .find(|attr| attr.name.local.to_string() == "rel");
                    let hrefs = attrs
                        .iter()
                        .find(|attr| attr.name.local.to_string() == "href");
                    match (rels, hrefs) {
                        (Some(rel), Some(href)) => {
                            if !rel.value.to_string().contains("tag") {
                                result.push_str("[[");
                                result.push_str(&href.value);
                                result.push_str("][");
                                keep_going!(input, result);
                                result.push_str("]]");
                            } else {
                                keep_going!(input, result);
                            }
                        }
                        _ => keep_going!(input, result),
                    }
                }
                _ => {}
            }
        }
        _ => {
            keep_going!(input, result);
        }
    };
}

impl Org {
    pub(crate) fn new_from_config(config: &OrgConfig) -> Org {
        let now = Utc::now();
        let filename = format!("{:>04}{:>02}{:>02}.org", now.year(), now.month(), now.day());
        let date = format!("{:>04}-{:>02}-{:>02}", now.year(), now.month(), now.day());
        let full_path = Path::new(&config.location).join(&filename);
        log::debug!("Org file: {}", full_path.to_string_lossy());

        Org {
            path: Path::new(&config.location).to_path_buf(),
            filename: filename,
            date,
        }
    }

    /// Creates the title (entry) for the record
    fn title(record: &Data) -> String {
        return format!("* {user}/{id}", user = record.account, id = record.id);
    }

    /// Creates the body of the markdown content from the incoming data
    fn body(record: &Data) -> String {
        let dom = parse_document(RcDom::default(), Default::default())
            .from_utf8()
            .read_from(&mut record.text.as_bytes())
            .unwrap();
        let mut result = String::new();
        result.push_str("  "); // initial identantion
        walk(&dom.document, &mut result);
        result.trim().into()
    }
}

impl Storage for Org {
    fn save(&self, record: &Data) {
        let org_file = self.path.join(&self.filename);
        let mut fp = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&org_file)
            .unwrap_or_else(|_| {
                // Let's assume here that the problem is that the file doesn't exist.
                log::debug!(
                    "Creating {filename}",
                    filename = &org_file.to_string_lossy()
                );
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(&org_file)
                    .map(|mut fp| {
                        let text =
                            format!("#+title: Favourites from {date}\n\n", date = &self.date);
                        fp.write_all(text.as_bytes()).unwrap();
                        fp
                    })
                    .unwrap()
            });
        fp.write_all(Org::title(record).as_bytes()).unwrap();
        fp.write_all("\n".as_bytes()).unwrap();
        fp.write_all(Org::body(record).as_bytes()).unwrap();
        fp.write_all("\n\n".as_bytes()).unwrap();

        if !record.attachments.is_empty() {
            fp.write_all("  Attachments:\n".as_bytes()).unwrap();
            for attachment in record.attachments.iter() {
                let filename = attachment.filename().to_string();
                let in_storage = self.path.join(&record.id).join(&filename);
                let mut target = File::create(&in_storage).unwrap();
                log::debug!(
                    "Downloading attachment {} as {}",
                    filename,
                    in_storage.to_string_lossy()
                );
                attachment.download().copy_to(&mut target).unwrap();

                let attachment_info =
                    format!("  - [[{}][{}]\n", in_storage.to_string_lossy(), filename);
                fp.write_all(attachment_info.as_bytes()).unwrap();
            }
            fp.write_all("\n\n".as_bytes()).unwrap()
        }
    }
}
