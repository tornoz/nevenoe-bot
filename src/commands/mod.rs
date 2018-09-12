mod termofis;
mod glosbe;
mod languagetool;

pub use self::termofis::run as termofis_run;
pub use self::glosbe::run as glosbe_run;
pub use self::languagetool::run as languagetool_run;
