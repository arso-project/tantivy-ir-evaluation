use std::fs;
use std::path::Path;
use tantivy::Index;
use tantivy::Result;
use tantivy::schema::*;

#[derive(Debug)]
pub struct article {
  title: String,
  body: String,
}

fn read_xml(filepath : Path) -> Vec<article> {
  let mut articles: Vec<article> = Vec::new(); 
  let content = fs::read_to_string(filepath).unwrap();
  let root: Element = content.parse().unwrap();
  for child in root.children() {
      if child.name() == "REUTERS" {
	  let mut title = String::new();
	  let mut body = String::new();
	  for article_child in child.children() {
		if article_child.name() == "TEXT" {
			for text_child in article_child.children() {
				if text_child.name() == "TITLE" {
					title = text_child.text();
				} else if text_child.name() == "BODY" {
					body = text_child.text();
				}
			}
		}
	  }
	  articles.push(ReutersArticle {
		title: title, 
		body: body,
	  }); 
      }
  }
  return articles;
}

fn create_index() -> Result<tantivy::Index> {
	// create schema
	let mut schema_builder = SchemaBuilder::default();
	schema_builder.add_text_field("title", TEXT | STORED);
	schema_builder.add_text_field("body", TEXT | STORED);
	let schema = schema_builder.build();

	// create index	
	let index = match Index::create_in_dir(Path::new("./index/"), schema.clone()) {
		Ok(index) => index,
		Err(e)    => panic!("Error: {}", e),
	};
	Ok(index)
}

fn index_articles(index: tantivy::Index, articles: Vec<ReutersArticle>) -> Result<()> {
	let mut index_writer = index.writer(50_000_000)?;
	let schema = index.schema();
	let title = schema.get_field("title").unwrap();
	let body = schema.get_field("body").unwrap();

	for article in articles {
		println!("Title{},title");
		let mut doc = Document::default();
		doc.add_text(title, &article.title);
		doc.add_text(body, &article.body);
		index_writer.add_document(doc);
	}
	
	index_writer.commit();

	Ok(())	
}

fn main() {
	let articles = read_xml();
	
	let index = create_index().unwrap();
	index_articles(index, articles);
}
