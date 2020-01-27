use tantivy::schema::*;
use tantivy::Result;

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn benchmarkreader(dataset: &str) -> Result<HashMap<String, Vec<i32>>> {
    let mut store = HashMap::new();
    let f = File::open(dataset).unwrap();
    let file = BufReader::new(&f);
    let re = Regex::new(r"[\t]+").unwrap();
    let re_numbers = Regex::new(" ").unwrap();
    for line in file.lines() {
        let mut values: Vec<i32> = Vec::new();
        let uline = line.unwrap();
        let entry: Vec<&str> = re.split(&uline).collect();
        let key = &entry[0].to_string();
        let pre_values: Vec<&str> = re_numbers.split(entry[1]).collect();
        for value in pre_values {
            let value = value.parse::<i32>().unwrap();

            values.push(value);
        }
        store.insert(key.clone(), values.clone());
    }
    Ok(store)
}

pub fn reader(dataset: &str) -> Vec<Vec<(String, Value)>> {
    let mut articles: Vec<Vec<(String, Value)>> = Vec::new();
    let f = File::open(dataset).unwrap();
    let file = BufReader::new(&f);
    let re = Regex::new(r"[\t]+").unwrap();
    let mut count = 1;
    for line in file.lines() {
        let mut article: Vec<(String, Value)> = Vec::new();
        let uline = line.unwrap();
        let entry: Vec<&str> = re.split(&uline).collect();
        let title = ("title".to_string(), Value::Str(entry[0].to_string()));
        article.push(title);
        let body = ("body".to_string(), Value::Str(entry[1].to_string()));
        article.push(body);
        let fulltext = (
            "fulltext".to_string(),
            Value::Str(entry[0].to_string() + "\n" + entry[1]),
        );
        article.push(fulltext);
        let id = ("id".to_string(), Value::U64(count));
        article.push(id);
        count += 1;
        articles.push(article);
    }

    articles
}
