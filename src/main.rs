use minidom::Element;
use std::fs;

#[derive(Debug)]
pub struct ReutersArticle {
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

fn main() {
	let articles = read_xml();
	println!("{:?}", articles);
}
