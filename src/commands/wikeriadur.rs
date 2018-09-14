use html2runes::markdown;
use reqwest;
use regex::Regex;
use std::error::Error;

static WIKERIADUR_URL: &'static str = "https://br.wiktionary.org/wiki/%term%?action=raw";

// pub fn run(term: &str) -> Result<String, &Error> {
pub fn run(term: &str) -> String {
    let uri = str::replace(WIKERIADUR_URL, "%term%", &term);

    println!("{}", &uri);
    let mut res = reqwest::get(&uri).unwrap();
    let text = markdown::convert_string(&res.text().unwrap());

    let re = Regex::new(r"'''(.*)'''(.*)#(.*)#(.*)").unwrap();
    //let after = re.replace_all(&text, "888 $word 888");
    for cap in re.captures_iter(&text) {
        println!("CAP1: {}, CAP2: {}, CAP3: {}, CAP4: {} ", &cap[1], &cap[2], &cap[3], &cap[4]);
    }
    //println!("{}", after);
    
    return "pomme".to_string();
}
