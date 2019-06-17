use minidom::Element;
use std::fs;
use tantivy::schema::*;

//use chrono::{DateTime, FixedOffset, TimeZone};

pub fn reader() -> Vec<Vec<(String, Value)>> {
	let mut articles: Vec<Vec<(String, Value)>> = Vec::new();

	let content = fs::read_to_string("datasets/reut2-003.sgm").unwrap();
	let root: Element = content.parse().unwrap();
	for child in root.children() {
		let mut article: Vec<(String, Value)> = Vec::new();
		let mut dateline = String::new();
		if child.name() == "REUTERS" {
			let oldid = (
				r"oldid".to_string(),
				Value::U64(child.attr("OLDID").unwrap().parse::<u64>().unwrap()),
			);
			article.push(oldid);
			let newid = (
				r"newid".to_string(),
				Value::U64(child.attr("NEWID").unwrap().parse::<u64>().unwrap()),
			);
			article.push(newid);
			//let mut date = FixedOffset::west(0).ymd(0,0,0).and_hms(0,0,0);
			for article_child in child.children() {
				if article_child.name() == "TEXT" {
					for text_child in article_child.children() {
						if text_child.name() == "TITLE" {
							let title = (r"title".to_string(), Value::Str(text_child.text()));
							article.push(title);
						} else if text_child.name() == "BODY" {
							let body = (r"body".to_string(), Value::Str(text_child.text()));
							article.push(body);
						} else if text_child.name() == "DATELINE" {
							dateline = text_child.text();
						}
					}
				} else if article_child.name() == "DATE" {
					//date = DateTime::parse_from_str(&article_child.text(), "%d-%b-%Y %H:%M:%S.2f").unwrap();
				}
			}

			articles.push(article);
		};
	}
	return articles;
}

/*
moved to index.rs and main.rs

pub fn create_index() -> Result<tantivy::Index> {
	// create schema
	let mut schema_builder = SchemaBuilder::default();
	schema_builder.add_u64_field("oldid", INDEXED | STORED);
	schema_builder.add_u64_field("newid", INDEXED | STORED);
	schema_builder.add_u64_field("date", INDEXED | STORED);
	schema_builder.add_text_field("title", TEXT | STORED);
	schema_builder.add_text_field("dateline", TEXT | STORED);
	schema_builder.add_text_field("body", TEXT | STORED);
	let schema = schema_builder.build();

	// create index
	let index = match Index::create_in_dir(Path::new("./index/"), schema.clone()) {
		Ok(index) => index,
		Err(e)    => panic!("Error: {}", e),
	};
	Ok(index)
}

pub fn index_articles(index: tantivy::Index, articles: Vec<ReutersArticle>) -> Result<()> {
	let mut index_writer = index.writer(50_000_000)?;
	let schema = index.schema();
	let title = schema.get_field("title").unwrap();
	let body = schema.get_field("body").unwrap();

	for article in articles {
		let mut doc = Document::default();
		doc.add_text(title, &article.title);
		doc.add_text(body, &article.body);
		index_writer.add_document(doc);
	}
	index_writer.commit()?;


	Ok(())
}
 */
