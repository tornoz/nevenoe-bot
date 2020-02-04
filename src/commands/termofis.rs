
use html2runes::markdown;
use reqwest;
use regex::Regex;

static TERMOFIS_URL: &'static str = "http://www.fr.brezhoneg.bzh/include/ajax/ajax.rechercheTermofis.php?logSearch=true&TERME=%term%&NOM=0&TER_DOMAINE=&LANGUE=_FR&TPLCODE=TPL_TERMOFIS&isSearch=true&numPage=1&IDSEARCH=";
static CLEANING_REGEX: & 'static str = r"(?mu)(\[\s\]\(javascript:void\(\)\))|(\(#[a-zA-Z_]*\))|(Résultats: [0-9]* sur [0-9]*)|(\[\(.*\)\])";

// pub fn run(term: &str) -> Result<String, &Error> {
pub fn run(term: &str) -> String {
    let uri = str::replace(TERMOFIS_URL, "%term%", &term);
    let cleaning_re = Regex::new(CLEANING_REGEX).unwrap();

    println!("{}", &uri);
    let mut res = reqwest::get(&uri).unwrap();
    let mut result = String::from(markdown::convert_string(&res.text().unwrap()));
    result = String::from(cleaning_re.replace_all(&result, ""));
    result.truncate(1993);
    result.push_str("[...]");
    return result;
}
