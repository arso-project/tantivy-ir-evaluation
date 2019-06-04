use quick_xml::Reader;
use quick_xml::events::Event;
use std::path::Path;
use std::io;
use minidom::Element;
use std::fs;

//fn read_xml() {
  //let mut reader = match Reader::from_file(Path::new("datasets/reuters21578/reut2-000.sgm")) {
	//Err(e) => panic!("Something went wrong while reading xml file"),
	//Ok(f) => f,
  //};
  //reader.trim_text(true);
  
  
  //let mut count = 0;
  //let mut txt = Vec::new();
  //let mut buf = Vec::new();
  
  //// The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
  //loop {
      //let title = String::new();
      //let text = String::new();
      //match reader.read_event(&mut buf) {
          //Ok(Event::Start(ref e)) => {
              //match e.name() {
                  //b"REUTERS" => println!("attributes values: {:?}",
                                      //e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>()),
                  //b"REUTERS" => count += 1,
                  //_ => (),
              //}
          //},
          //Ok(Event::Text(e)) => {
		//match e.name() {
                  //b"TITLE" => title = e.unescape_and_decode(&reader).unwrap(),
		//}
	  //},
          //Ok(Event::Eof) => break, // exits the loop when reaching end of file
          //Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
          //_ => (), // There are several other `Event`s we do not consider here
      //}
  
      //// if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
      //buf.clear();
  //}
  //println!("{} Reuters articles", count);
  ////println!("{:?}", txt);
//}
pub struct ReutersArticle {
  title: String,
  body: String,
}

fn read_xml() {
  let content = fs::read_to_string("datasets/reuters21578/reut2-000.sgm").unwrap();
  //println!("{}", content);
let root: Element = content.parse().unwrap();
println!("{:?}", root);
for child in root.children() {
            if child.name() == "REUTERS" {
		println!("{:?}", child);
		} else {
		println!("{:?}", child);		
}
}
}

fn main() {
	read_xml();
}
