use minidom::Element;
use std::fs;
use std::path::Path;
use tantivy::Index;
use tantivy::Result;
use tantivy::schema::*;
//use chrono::{DateTime, FixedOffset, TimeZone};
use std::str::FromStr;

#[derive(Debug)]
pub struct ReutersArticle {
  oldid: u64,
  newid: u64,
  //date: DateTime<FixedOffset>,
  dateline: String,
  title: String,
  body: String,
}

fn read_xml() -> Vec<ReutersArticle> {
  let mut articles: Vec<ReutersArticle> = Vec::new(); 
  let content = fs::read_to_string("datasets/reuters21578/reut2-000.sgm").unwrap();
  let root: Element = content.parse().unwrap();
  for child in root.children() {
      if child.name() == "REUTERS" {
	  let mut title = String::new();
	  let mut body = String::new();
	  let mut dateline = String::new();
          let oldid = child.attr("OLDID").unwrap().parse::<u64>().unwrap();
          let newid = child.attr("NEWID").unwrap().parse::<u64>().unwrap();
          //let mut date = FixedOffset::west(0).ymd(0,0,0).and_hms(0,0,0);
	  for article_child in child.children() {
		if article_child.name() == "TEXT" {
			for text_child in article_child.children() {
				if text_child.name() == "TITLE" {
					title = text_child.text();
				} else if text_child.name() == "BODY" {
					body = text_child.text();
                                } else if text_child.name() == "DATELINE" {
                                        dateline = text_child.text();
                                }
			}
                } else if article_child.name() == "DATE" {
                    //date = DateTime::parse_from_str(&article_child.text(), "%d-%b-%Y %H:%M:%S.2f").unwrap();
                }
	  }
	  articles.push(ReutersArticle {
                oldid: oldid,
                newid: newid,
                //date: date,
                dateline: dateline,
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

fn index_articles(index: tantivy::Index, articles: Vec<ReutersArticle>) -> Result<()> {
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
	
	index_writer.commit();

	Ok(())	
}

fn main() {
        // TODO: Parse all xml files
        // TODO: Parse and index oldid, newid, date, dateline
	let articles = read_xml();
	
	let index = create_index().unwrap();
	index_articles(index, articles);
}
