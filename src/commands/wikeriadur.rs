use reqwest;
use regex::Regex;
use std::error::Error;

static WIKERIADUR_URL: &'static str = "https://br.wiktionary.org/wiki/%term%?action=raw";
static langRegex: &'static str = r"(?m)\{\{=([a-z]{2,3})=\}\}";
static classRegex: &'static str = r"(?m)\{\{-([[:alpha:]-]*)-\|[a-z]{2,3}.{0,}\}\}";
static definitionRegex: &'static str = r"(?m)^#[^*](.*)$";
static exampleRegex: &'static str = r"(?m)^#\*(.*)$";

// pub fn run(term: &str) -> Result<String, &Error> {
pub fn run(term: &str) -> String {
    let uri = str::replace(WIKERIADUR_URL, "%term%", &term);
    let mut lang: &str;
    let mut wordClass: &str;
    let mut definition: &str;
    let mut example: &str;

    let langRe = Regex::new(langRegex).unwrap();
    let classRe = Regex::new(classRegex).unwrap();
    let definitionRe = Regex::new(definitionRegex).unwrap();
    let exampleRe = Regex::new(exampleRegex).unwrap();

    println!("{}", &uri);
    let mut res = reqwest::get(&uri).unwrap();
    let text = &res.text().unwrap();
    let mut lines:Vec<&str> = text.split("\n").collect();

    for line in &lines {
        //println!("LINE: {} ", line);
        for cap in langRe.captures_iter(line) {
            println!("LANG: {} ", &cap[1]);
        }

        for cap in classRe.captures_iter(line) {
            println!("CLASS: {} ", &cap[1]);
        }

        for cap in definitionRe.captures_iter(line) {
            println!("DEFINITION: {} ", &cap[1]);
        }

        for cap in exampleRe.captures_iter(line) {
            println!("EXAMPLE: {} ", &cap[1]);
        }
    }

    //println!("{}", after);

    return "text".to_string();
}
