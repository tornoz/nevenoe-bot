use reqwest;
use regex::Regex;
use std::error::Error;

static WIKERIADUR_URL: &'static str = "https://br.wiktionary.org/wiki/%term%?action=raw";
static langRegex: &'static str = r"(?m)\{\{=([a-z]{2,3})=\}\}";
static classRegex: &'static str = r"(?m)\{\{-([[:alpha:]-]*)-\|[a-z]{2,3}.{0,}\}\}";
static definitionRegex: &'static str = r"(?m)^#[^*](.*)$";
static exampleRegex: &'static str = r"(?m)^#\*(.*)\{\{mammenn\|.*$";

static mutatedWordLinkRegex: & 'static str = r"(?m)\[\[[[:alpha:]\-\s']*\|([[:alpha:]\-\s']*)\]\]";
static langWordLinkRegex: & 'static str = r"(?m)\{\{ucf\|([[:alpha:]\-\s']*)\}\}";
static linkRegex: & 'static str = r"(?m)\[\[([[:alpha:]\-\s']*)\]\]";
static quoteRegex: & 'static str = r"(?m)([']{2,})";

// pub fn run(term: &str) -> Result<String, &Error> {
pub fn run(term: &str) -> String {
    let uri = str::replace(WIKERIADUR_URL, "%term%", &term);
    let mut result = String::from("");

    // identification regexes
    let langRe = Regex::new(langRegex).unwrap();
    let classRe = Regex::new(classRegex).unwrap();
    let definitionRe = Regex::new(definitionRegex).unwrap();
    let exampleRe = Regex::new(exampleRegex).unwrap();

    // replacement regexes
    let mutatedWordLinkRe = Regex::new(mutatedWordLinkRegex).unwrap();
    let langWordLinkRe = Regex::new(langWordLinkRegex).unwrap();
    let linkRe = Regex::new(linkRegex).unwrap();
    let quoteRe = Regex::new(quoteRegex).unwrap();

    println!("{}", &uri);
    let mut res = reqwest::get(&uri).unwrap();
    let text = &res.text().unwrap();
    let mut lines:Vec<&str> = text.split("\n").collect();

    let mut capturing = false;

    for mut line in &lines {
        let mut wordClass = String::from("");
        let mut definition = String::from("");
        let mut example = String::from("");
        let mut lang = String::from("");
        let mut toPrint = false;

        for cap in langRe.captures_iter(line) {
            if cap[1] == "br".to_string() {
                lang = String::from(&cap[1]);
                capturing = true;
            } else {
                capturing = false;
            }
        }

        if capturing {
            for cap in classRe.captures_iter(line) {
                toPrint = true;
                let mut afterLine = mutatedWordLinkRe.replace_all(&cap[1], "$1");
                let mut afterLine = langWordLinkRe.replace_all(&afterLine, "$1");
                let mut afterLine = linkRe.replace_all(&afterLine, "$1");
                let mut afterLine = quoteRe.replace_all(&afterLine, "");
                wordClass = String::from(afterLine);
                //println!("CLASS: {} ", &cap[1]);
            }

            for cap in definitionRe.captures_iter(line) {
                toPrint = true;
                let mut afterLine = mutatedWordLinkRe.replace_all(&cap[1], "$1");
                let mut afterLine = langWordLinkRe.replace_all(&afterLine, "$1");
                let mut afterLine = linkRe.replace_all(&afterLine, "$1");
                let mut afterLine = quoteRe.replace_all(&afterLine, "");
                definition = String::from(afterLine);
            }

            for cap in exampleRe.captures_iter(line) {
                toPrint = true;
                let mut afterLine = mutatedWordLinkRe.replace_all(&cap[1], "$1");
                let mut afterLine = langWordLinkRe.replace_all(&afterLine, "$1");
                let mut afterLine = linkRe.replace_all(&afterLine, "$1");
                let mut afterLine = quoteRe.replace_all(&afterLine, "");
                example = String::from(afterLine);
            }
        }

        if toPrint {
            result.push_str(&lang);
            result.push_str(&wordClass);
            result.push_str(&definition);
            result.push_str(&example);
        }
    }

    //println!("{}", after);
    return result;
}
