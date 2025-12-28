macro_rules! deps {
    () => {
        Value!();
        Table!();
    };
}

macro_rules! toml {
    () => {
        deps!();
        # [doc = " Construct a [`Table`] from TOML syntax."] # [doc = ""] # [doc = " ```rust"] # [doc = " let cargo_toml = toml::toml! {"] # [doc = "     [package]"] # [doc = "     name = \"toml\""] # [doc = ""] # [doc = "     [dependencies]"] # [doc = "     serde = \"1.0\""] # [doc = ""] # [doc = "     [dev-dependencies]"] # [doc = "     serde_derive = \"1.0\""] # [doc = "     serde_json = \"1.0\""] # [doc = " };"] # [doc = ""] # [doc = " println!(\"{:#?}\", cargo_toml);"] # [doc = " ```"] # [macro_export] macro_rules ! toml { ($ ($ toml : tt) +) => { { let table = $ crate :: value :: Table :: new () ; let mut root = $ crate :: Value :: Table (table) ; $ crate :: toml_internal ! (@ toplevel root [] $ ($ toml) +) ; match root { $ crate :: Value :: Table (table) => table , _ => unreachable ! () , } } } ; }
    };
}

toml!();