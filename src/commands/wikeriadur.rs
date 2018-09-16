use reqwest;
use regex::Regex;
use std::error::Error;

static WIKERIADUR_URL: &'static str = "https://br.wiktionary.org/wiki/%term%?action=raw";
static langRegex: &'static str = r"(?m)\{\{=([a-z]{2,3})=\}\}"; // matches the lang tag
static classRegex: &'static str = r"(?mu)\{\{-([[:alpha:]ñùê-]*)-\|[a-z]{2,3}.{0,}\}\}"; // matches the grammatical class of the word (beginning of definition)
static definitionRegex: &'static str = r"(?m)^#([^*].*)$"; // matches the definition
static exampleRegex: &'static str = r"(?m)^#\*(.*)\{\{mammenn\|.*$"; // matches the use case example

static mutatedWordLinkRegex: & 'static str = r"(?mu)\[\[[[:alpha:]ñùê\-\s']*\|([[:alpha:]ñùê\-\s']*)\]\]";
static langWordLinkRegex: & 'static str = r"(?mu)\{\{ucf\|([[:alpha:]ñùê\-\s']*)\}\}";
static wikiLinkRegex: & 'static str = r"(?mu)\[\[w:[[:alpha:]ñùê\-\s']*\|([[:alpha:]ñùê\-\s']*)\]\]";
static linkRegex: & 'static str = r"(?mu)\[\[([[:alpha:]ñùê\-\s']*)\]\]";
static quoteRegex: & 'static str = r"(?m)([']{2,})";

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
    let wikiLinkRe = Regex::new(wikiLinkRegex).unwrap();
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
                let mut afterLine = wikiLinkRe.replace_all(&afterLine, "$1");
                let mut afterLine = langWordLinkRe.replace_all(&afterLine, "$1");
                let mut afterLine = linkRe.replace_all(&afterLine, "$1");
                let mut afterLine = quoteRe.replace_all(&afterLine, "");
                wordClass = String::from(afterLine);
                //println!("CLASS: {} ", &cap[1]);
                result.push_str("\n\n");
                result.push_str(&wordClass);
            }

            for cap in definitionRe.captures_iter(line) {
                toPrint = true;
                let mut afterLine = mutatedWordLinkRe.replace_all(&cap[1], "$1");
                let mut afterLine = wikiLinkRe.replace_all(&afterLine, "$1");
                let mut afterLine = langWordLinkRe.replace_all(&afterLine, "$1");
                let mut afterLine = linkRe.replace_all(&afterLine, "$1");
                let mut afterLine = quoteRe.replace_all(&afterLine, "");
                definition = String::from(afterLine);
                result.push_str("\n -");
                result.push_str(&definition);
            }

            for cap in exampleRe.captures_iter(line) {
                toPrint = true;
                let mut afterLine = mutatedWordLinkRe.replace_all(&cap[1], "$1");
                let mut afterLine = wikiLinkRe.replace_all(&afterLine, "$1");
                let mut afterLine = langWordLinkRe.replace_all(&afterLine, "$1");
                let mut afterLine = linkRe.replace_all(&afterLine, "$1");
                let mut afterLine = quoteRe.replace_all(&afterLine, "");
                example = String::from(afterLine);
                result.push_str("\n ----");
                result.push_str(&example);
            }
        }
    }

    //println!("{}", after);
    return result;
}
