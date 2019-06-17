
mod reuters_importer;
mod index;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use tantivy::schema::FieldValue;
use tantivy::{Document, Index, Result, TantivyError};

use serde_json;
use serde::{Deserialize, Serialize};
use std::io::BufReader;





fn main() {
        // TODO: Parse all xml files
        // TODO: Parse and index oldid, newid, date, dateline
	let base_path = PathBuf::from(r"./index");
	let mut file = File::open(r"./schemata/reuters.json").unwrap();
	let mut buf_reader = BufReader::new(file);
  let mut contents = String::new();
  buf_reader.read_to_string(&mut contents).unwrap();
	let schema : tantivy::schema::Schema =
  serde_json::from_str(&contents).expect("JSON was not well-formatted");
		
	
	let mut catalog = index::IndexCatalog::new(base_path).unwrap();
	catalog.create_index("reuters".to_string(), schema).unwrap();
	let index = catalog.get_index(&"reuters".to_string()).unwrap();
		let articles = reuters_importer::reader();
	index.add_documents(&articles);

	
	


}



