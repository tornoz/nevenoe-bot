use reqwest;
use serde_json::{Value};
use regex::Regex;

static GLOSBE_URL_API: &'static str = "https://glosbe.com/gapi/translate?from=fra&dest=bre&format=json&phrase=%term%";
static GLOSBE_URL_SITE: &'static str = "https://glosbe.com/fr/br/%term%";

static WORD_REGEX: & 'static str = r#"(?mU)<strong class=" phr">(.*)</strong>"#;

#[derive(Deserialize)]
struct ResultGlosbe {
    result: String,
    tuc: Vec<Value>
}
// struct GlosbeError {}
//
// impl Error for GlosbeError {
//     fn description(&self) -> &str {
//         "Result not ok"
//     }
// }

// pub fn run(term: &str) -> Result<String, &Error> {
pub fn run(term: &str) -> String {
    let mut res = get_from_site(term);
    return res;
}

fn get_from_api(term: &str) -> String {
    let uri = str::replace(GLOSBE_URL_API, "%term%", &term);
    let mut res = reqwest::get(&uri).unwrap();

    let json_data: ResultGlosbe = res.json().unwrap();
    if json_data.result == "ok" {
        let mut message: String = String::new();
        message.push_str("Resultat:");
        for i in json_data.tuc {
            let mut word =  &i["phrase"]["text"];
            if word.is_string() {
                message.push_str(&format!("\n **{}**", &str::replace(word.as_str().unwrap(), "\"", "")));
            }
        };
        // return Ok(message);
        return message;
    }

    return String::from("oups");
}

fn get_from_site(term: &str) -> String {
    let uri = str::replace(GLOSBE_URL_SITE, "%term%", &term);
    let mut res = reqwest::get(&uri);
    let word_re = Regex::new(WORD_REGEX).unwrap();

    let mut buffer = String::from("");
    let text = &res.unwrap().text().unwrap();

    let result = word_re.captures_iter(text);

    for mat in result {
        buffer.push_str("‣ ");
        buffer.push_str(&mat[1]);
        buffer.push_str("\n");
    }

    return buffer;
}
