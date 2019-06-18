
mod reuters_importer;
mod moviedb_importer;
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
	
	let schema = read_schema("./schemata/movies.json".to_string());

	
	let mut catalog = index::IndexCatalog::new(base_path).unwrap();
	catalog.create_index("movies".to_string(), schema).unwrap();
	let index = catalog.get_index(&"movies".to_string()).unwrap();
	let articles = moviedb_importer::reader();
	println!("Articles: {:?}",&articles);
	
	let documents = match index.add_documents(&articles){
		Ok(documents) => documents,
		Err(e)    => panic!("Error: {}", e),
	};
}

fn read_schema(path : String) -> tantivy::schema::Schema{
	 let mut file = File::open(path).unwrap();
	let mut buf_reader = BufReader::new(file);
  let mut contents = String::new();
  buf_reader.read_to_string(&mut contents).unwrap();
	let schema : tantivy::schema::Schema =
  serde_json::from_str(&contents).expect("JSON was not well-formatted");
	return schema
}



