use reqwest;
use serde_json::{Value, Error as SerdeError};
use std::error::Error;

static GLOBSE_URL: &'static str = "https://glosbe.com/gapi/translate?from=fra&dest=bre&format=json&phrase=%term%";

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

pub fn run(term: &str) -> Result<String, &Error> {

    let uri = str::replace(GLOBSE_URL, "%term%", &term);
    let mut res = reqwest::get(&uri).unwrap();

    let mut json_data: ResultGlosbe= res.json().unwrap();
    if(json_data.result == "ok") {
        let mut message: String = String::new();
        message.push_str("Resultat:");
        for i in json_data.tuc {
            let mut word =  &i["phrase"]["text"];
            if word.is_string() {
                message.push_str(&format!("\n **{}**", &str::replace(word.as_str().unwrap(), "\"", "")));
            }
        };
        return Ok(message);
    } else {
        return Err(Error {});
    }
}
