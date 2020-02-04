use reqwest;
use regex::Regex;
use std::collections::HashMap;

static WIKERIADUR_URL: &'static str = "https://br.wiktionary.org/wiki/%term%?action=raw";
static LANG_REGEX: &'static str = r"(?m)\{\{=([a-z]{2,3})=\}\}"; // matches the lang tag
static CLASS_REGEX: &'static str = r"(?mu)\{\{-([[:alpha:]ñùê-]*)-\|[a-z]{2,3}.{0,}\}\}"; // matches the grammatical class of the word (beginning of definition)
static DEFINITION_REGEX: &'static str = r"(?m)^#([^:*].*)$"; // matches the definition
static EXAMPLE_REGEX: &'static str = r"(?m)^#[:]{0,1}\*(.*)\{\{mammenn\|.*$"; // matches the use case example

static MUTATED_WORD_LINK_REGEX: & 'static str = r"(?mu)\[\[[[:alpha:]ñùê\-\s']*\|([[:alpha:]ñùê\-\s']*)\]\]";
static LANG_WORD_LINK_REGEX: & 'static str = r"(?mu)\{\{ucf\|([[:alpha:]ñùê\-\s']*)\}\}";
static WIKI_LINK_REGEX: & 'static str = r"(?mu)\[\[w:[[:alpha:]ñùê\-\s']*\|([[:alpha:]ñùê\-\s']*)\]\]";
static LINK_REGEX: & 'static str = r"(?mu)\[\[([[:alpha:]ñùê\-\s']*)\]\]";
static QUOTE_REGEX: & 'static str = r"(?m)([']{2,})";

pub fn run(term: &str) -> String {
    let mut word_classes: HashMap<String, String> = HashMap::new();
    word_classes.insert("ak".to_string(), "Anv-Kadarn".to_string());
    word_classes.insert("rag-gour".to_string(), "Raganv-Gour".to_string());
    word_classes.insert("furm-ak".to_string(), "Furm Anv-Kadarn".to_string());
    word_classes.insert("ag".to_string(), "Anv-Gwan".to_string());
    word_classes.insert("ag-pet".to_string(), "Anv-Gwan Petvediñ".to_string());
    word_classes.insert("verb".to_string(), "Verb".to_string());

    let uri = str::replace(WIKERIADUR_URL, "%term%", &term);
    let mut result = String::from("");

    // identification regexes
    let lang_re = Regex::new(LANG_REGEX).unwrap();
    let class_re = Regex::new(CLASS_REGEX).unwrap();
    let definition_re = Regex::new(DEFINITION_REGEX).unwrap();
    let example_re = Regex::new(EXAMPLE_REGEX).unwrap();

    // replacement regexes
    let mutated_word_link_re = Regex::new(MUTATED_WORD_LINK_REGEX).unwrap();
    let lang_word_link_re = Regex::new(LANG_WORD_LINK_REGEX).unwrap();
    let wiki_link_re = Regex::new(WIKI_LINK_REGEX).unwrap();
    let link_re = Regex::new(LINK_REGEX).unwrap();
    let quote_re = Regex::new(QUOTE_REGEX).unwrap();

    let mut res = reqwest::get(&uri);
    if let Err(_e) = res {

    } else {
        let mut buffer = String::from("");
        let text = &res.unwrap().text().unwrap();
        let mut lines:Vec<&str> = text.split("\n").collect();

        let mut capturing = false;
        let mut to_print = false;

        for mut line in &lines {
            let mut word_class = String::from("");
            let mut definition = String::from("");
            let mut example = String::from("");
            let mut lang = String::from("");

            for cap in lang_re.captures_iter(line) {
                if cap[1] == "br".to_string() {
                    lang = String::from(&cap[1]);
                    capturing = true;
                } else {
                    capturing = false;
                }
            }

            if capturing {
                for cap in class_re.captures_iter(line) {
                    to_print = true;
                    let mut after_line = mutated_word_link_re.replace_all(&cap[1], "$1");
                    let mut after_line = wiki_link_re.replace_all(&after_line, "$1");
                    let mut after_line = lang_word_link_re.replace_all(&after_line, "$1");
                    let mut after_line = link_re.replace_all(&after_line, "$1");
                    let mut after_line = quote_re.replace_all(&after_line, "");
                    word_class = String::from(after_line);

                    buffer.push_str("\n= ");
                    match word_classes.get(&word_class) {
                        Some(full_class) => buffer.push_str(&full_class),
                        None => buffer.push_str(&word_class)
                    }
                    buffer.push_str(" =");
                }

                for cap in definition_re.captures_iter(line) {
                    to_print = true;
                    let mut after_line = mutated_word_link_re.replace_all(&cap[1], "$1");
                    let mut after_line = wiki_link_re.replace_all(&after_line, "$1");
                    let mut after_line = lang_word_link_re.replace_all(&after_line, "$1");
                    let mut after_line = link_re.replace_all(&after_line, "$1");
                    let mut after_line = quote_re.replace_all(&after_line, "");
                    definition = String::from(after_line);
                    buffer.push_str("\n •");
                    buffer.push_str(&definition);
                }

                for cap in example_re.captures_iter(line) {
                    to_print = true;
                    let mut after_line = mutated_word_link_re.replace_all(&cap[1], "$1");
                    let mut after_line = wiki_link_re.replace_all(&after_line, "$1");
                    let mut after_line = lang_word_link_re.replace_all(&after_line, "$1");
                    let mut after_line = link_re.replace_all(&after_line, "$1");
                    let mut after_line = quote_re.replace_all(&after_line, "");
                    example = String::from(after_line);
                    buffer.push_str("\n \t‣");
                    buffer.push_str(&example);
                }
            }
        }

        if to_print {
            result.push_str("```asciidoc\n[");
            result.push_str(term);
            result.push_str("]\n");
            result.push_str(&buffer);
            result.truncate(1990);
            result.push_str("[...]");
            result.push_str("\n```");
        }
    }

    if result == String::from("") {
        result = "No result".to_string();
    }
    return result
}
