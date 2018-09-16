
use html2runes::markdown;
use reqwest;

static TERMOFIS_URL: &'static str = "http://www.fr.brezhoneg.bzh/include/ajax/ajax.rechercheTermofis.php?logSearch=true&TERME=%term%&NOM=0&TER_DOMAINE=&LANGUE=_FR&TPLCODE=TPL_TERMOFIS&isSearch=true&numPage=1&IDSEARCH=";

// pub fn run(term: &str) -> Result<String, &Error> {
pub fn run(term: &str) -> String {
    let uri = str::replace(TERMOFIS_URL, "%term%", &term);

    println!("{}", &uri);
    let mut res = reqwest::get(&uri).unwrap();
    return markdown::convert_string(&res.text().unwrap());
}
