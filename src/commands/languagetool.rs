use reqwest;
use std::string::String;

static LT_URL: &'static str = "https://languagetool.org/api/v2/check";

#[derive(Deserialize)]
struct ReplacementLT {
    value: String
}
#[derive(Deserialize)]
struct MatchLT {
    message: String,
    offset: i32,
    length: i32,
    replacements: Vec<ReplacementLT>

}
#[derive(Deserialize)]
struct ResultLT {
    matches: Vec<MatchLT>
}

pub fn run(term: &str) -> String {

    let language = "br";
    let disabled_categories = "CASING";
    let mut res = reqwest::Client::new()
        .post(LT_URL)
        .form(&[("text", &term), ("language", &language), ("disabledCategories", &disabled_categories)])
        .send()
        .unwrap();
    let json_data: ResultLT= res.json().unwrap();
    if json_data.matches.len() == 0 {
        return String::from("N'eus bet kavet fazi ebet 👍");
    }
    let mut offset = 0;

    let mut phrase = String::from(term.clone());
    // let mut iterable_phrase = phrase.clone();
    let mut _message: String = String::new();
    for i in &json_data.matches {
        phrase = insert_utf16(&phrase, (i.offset + offset) as usize, "~~");
        offset = offset+2;
        phrase = insert_utf16(&phrase, (i.offset + offset + i.length) as usize, "~~");
        offset = offset+2;
        let mut is_replacement = false;
        if &i.replacements.len() > &0 {
            is_replacement = true;
        }
        if is_replacement {
            phrase = insert_utf16(&phrase, (i.offset + i.length + offset) as usize, &format!("**{}**", &i.replacements.first().unwrap().value));
            offset = offset + 4 + i.replacements.first().unwrap().value.encode_utf16().count() as i32;
        }
    }
    let mut message = String::new();
    message.push_str(&phrase);
    message.push('\n');

    for i in &json_data.matches {
        message.push_str(&i.message);
        message.push('\n');
    }
    return message;
}

pub fn insert_utf16(string: &str, pos: usize, insert: &str) -> String{
    let stringutf16 = string.encode_utf16();
    let insertutf16 = insert.encode_utf16();
    let mut string_vector:Vec<u16> = stringutf16.map(|res| res).collect();
    let mut insert_vector:Vec<u16> = insertutf16.map(|res| res).collect();
    let mut last = string_vector.split_off(pos);
    string_vector.append(&mut insert_vector);
    string_vector.append(&mut last);
    return String::from_utf16(&string_vector).unwrap()
    ;


}
