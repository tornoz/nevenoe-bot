mod termofis;
mod glosbe;
mod languagetool;
mod troer;
mod wikeriadur;

pub use self::termofis::run as termofis_run;
pub use self::glosbe::run as glosbe_run;
pub use self::languagetool::run as languagetool_run;
pub use self::troer::run as troer_run;
pub use self::wikeriadur::run as wikeriadur_run;
