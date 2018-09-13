

use reqwest;
use std::error::Error;
use html5ever::parse_document;
use html5ever::rcdom::{Document, Doctype, Text, Comment, Element, RcDom, Handle};

use html5ever::tendril::TendrilSink;

static TROER_URL: &'static str = "http://www.fr.brezhoneg.bzh/42-traducteur-automatique.htm";

// pub fn run(term: &str) -> Result<String, &Error> {
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

    let res = class_finder(dom.document, &xpath, false);
    // return value.into_string();
    return String::from(res);
    // let html = &res.text().unwrap();
}


pub fn class_finder(handle: Handle, class: &String, found: bool) -> &str
{
    let node = handle.borrow();
    let mut found = false;
    match node.node {
        //We found it
        Text (ref text)
            => if found {return &text.to_string();} else {return std::ptr::null()},

        //Check if in xpath
        Element ( ref name, _, ref attrs) => {
            println!("<{}", name.local);
            for attr in attrs.iter() {
                print!(" {}=\"{}\"", attr.name.local, attr.value);
                if(attr.name.local.to_string() == "class" && attr.value.to_string() == *class) {
                    found = true;
                }
            }
            for child in node.children.iter() {
                let res = class_finder(child.clone(),  &class, found);
                if res != "" {
                    return res;
                }
            }
            return "";
        },
        Document => {
            println!("Document");
            for child in node.children.iter() {
                return class_finder(child.clone(), &class, false);
            }
            return "";
        },
        Doctype (_,_,_) => { return ""},
        Comment (_) => { return ""}
    }

    //
}
