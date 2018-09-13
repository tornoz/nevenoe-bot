

use reqwest;
use std::error::Error;
use html5ever::parse_document;
use html5ever::rcdom::{Document, Doctype, Text, Comment, Element, RcDom, Handle};
use std::error;
use html5ever::tendril::TendrilSink;

static TROER_URL: &'static str = "http://www.fr.brezhoneg.bzh/42-traducteur-automatique.htm";

pub fn run(term: &str) -> String {

    let traduire = "Traduire";
    let mut res = reqwest::Client::new()
        .post(TROER_URL)
        .form(&[("idtf", "42"), ("TRADUCTION", term), ("TRADUIRE", traduire)])
        .send()
        .unwrap();
    let dom = parse_document(RcDom::default(), Default::default())
            .from_utf8()
            .read_from(&mut res.text().unwrap().as_bytes())
            .unwrap();
        let xpath = String::from("resultats");

    match class_finder(dom.document, &xpath, false) {
        Some(res) => return res,
        None => return String::from("oh no :(")
    }
}


pub fn class_finder(handle: Handle, class: &String, found: bool) -> Option<String>
{
    let node = handle.borrow();
    match node.node {
        //We found it
        Text (ref text)
            =>  {
                if found { Some(text.to_string())} else {None}
            },

        Element ( ref name, _, ref attrs) => {
            let mut found = false;
            for attr in attrs.iter() {
                if(attr.name.local.to_string() == "class" && attr.value.to_string() == *class) {
                    found = true;
                }
            }
            for child in node.children.iter() {
                if let Some(res) = class_finder(child.clone(),  &class, found) {
                    return Some(res);
                }
            }
            None
        },
        Document => {
            for child in node.children.iter() {
                if let Some(res) = class_finder(child.clone(),  &class, found) {
                    return Some(res);
                }
            }
            None
        },
        Doctype (_,_,_) => { None },
        Comment (_) => { None }
    }
}
