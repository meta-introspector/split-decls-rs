macro_rules! deps {
    () => {
        Config!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl ConfigTrait for Config { fn load () -> Self { let path = std :: env :: var ("TRAIT_FIXER_CONFIG") . unwrap_or_else (| _ | "rules.toml" . to_string ()) ; let content = std :: fs :: read_to_string (path) . expect ("Failed to read rules.toml") ; toml :: from_str (& content) . expect ("Invalid TOML in rules.toml") } fn get_rules (& self) -> & Vec < Rule > { & self . rule } }
    };
}

impl_1!();