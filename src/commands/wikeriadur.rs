use html2runes::markdown;
use reqwest;
use regex::Regex;
use std::error::Error;

static WIKERIADUR_URL: &'static str = "https://br.wiktionary.org/wiki/%term%?action=raw";
static langRegex = r"(?m){{=[a-z]{2,3}=}}";
static classRegex = r"(?m){{-([[:alpha:]-]*)-\|[a-z]{2,3}.{0,}}}";
static definitionRegex = r"(?m)^#[^*](.*)$";
static exampleRegex = r"(?m)^#\*(.*)$";

// pub fn run(term: &str) -> Result<String, &Error> {
pub fn run(term: &str) -> String {
    let uri = str::replace(WIKERIADUR_URL, "%term%", &term);

    println!("{}", &uri);
    let mut res = reqwest::get(&uri).unwrap();
    let text = markdown::convert_string(&res.text().unwrap());

    let re = Regex::new(r"'''(.*)'''(.*)#(.*)#(.*)").unwrap();

    // works with "gwalleur"
    for cap in re.captures_iter(&text) {
        println!("CAP1: {}, CAP2: {}, CAP3: {}, CAP4: {} ", &cap[1], &cap[2], &cap[3], &cap[4]);
    }
    //println!("{}", after);

    return text;
}
