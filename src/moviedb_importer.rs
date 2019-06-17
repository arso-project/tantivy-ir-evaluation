use std::path::Path;
use tantivy::Index;
use tantivy::Result;
use tantivy::schema::*;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use regex::Regex;


pub fn reader() -> Vec<Vec<(String, Value)>> {
let mut articles: Vec<Vec<(String, Value)>> = Vec::new();  
let f = File::open("datasets/movies.txt").unwrap();
let mut file = BufReader::new(&f);
let re = Regex::new(r"[\t]+").unwrap();
let mut count = 0;
    for line in file.lines() {
			let mut article: Vec<(String, Value)> = Vec::new();
			let uline = &line.unwrap();
			let entry : Vec<&str> = re.split(uline).collect();
			let title = ("title".to_string(),Value::Str(entry[0].to_string()));
			article.push(title);
			let body = ("body".to_string(),Value::Str(entry[1].to_string()));
			article.push(body);
			let id = ("id".to_string(), Value::U64(count));
			article.push(id);
			count += 1;
				
    }   

	
  return articles;
}


