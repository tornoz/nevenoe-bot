use reqwest;
use std::error::Error;
use victoria_dom::DOM;

static TROER_URL: &'static str = "http://www.fr.brezhoneg.bzh/42-traducteur-automatique.htm";

// pub fn run(term: &str) -> Result<String, &Error> {
pub fn run(term: &str) -> String {

    let traduire = "Traduire";
    let mut res = reqwest::Client::new()
        .post(TROER_URL)
        .form(&[("idtf", "42"), ("TRADUCTION", term), ("TRADUIRE", traduire)])
        .send()
        .unwrap();
        println!("allloo: ");
    let html = &res.text().unwrap();
    let dom = DOM::new(html);
    return dom.at("div.resultats").unwrap().text_all();
}
