#[cfg(feature = "translation")]
mod real {
    use include_dir::{include_dir, Dir};
    pub static LOCALES: Dir = include_dir!("st-locales");

    /// All locales in `st-locales` are loaded, with the default being `C.yml`, and then using
    /// standard locale terms: `en.yml` would work, or `en-US.yml`, or `zh-CN.yml`, etc. Each one
    /// of these contains a mapping of the [Words](crate::enums::Words) to the appropriate words
    /// that correspond in a given language. This allows dynamic translation of several
    /// grammatically-similar languages into native tongues without sacrificing structure or
    /// information.
    #[macro_export]
    macro_rules! load_locale {
        ($locale:expr) => {{
            use $crate::translator::LOCALES;

            let mut file = LOCALES.get_file(format!("{}.yml", $locale));
            if file.is_none() {
                file = LOCALES.get_file(format!("{}.yml", $locale.split("-").nth(0).unwrap()));
            }

            if file.is_none() {
                file = LOCALES.get_file("C.yml")
            }

            Translator::new(
                serde_yaml::from_str(
                    &file
                        .expect("Could not find any translations")
                        .contents_utf8()
                        .unwrap(),
                )
                .unwrap(),
            )
        }};
    }
}

#[cfg(feature = "translation")]
pub use self::real::*;
