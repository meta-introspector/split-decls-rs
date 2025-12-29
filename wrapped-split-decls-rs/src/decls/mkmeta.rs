// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "mkmeta",
decl_type: "function",
source_file: "./src/meta_pattern_visitor.rs",
source_crate: ".",
deps: [],
uses: ["ALL", "String"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! mkmeta {
    () => {
        macro_rules ! mkmeta { ($ enum_name : ident { $ ($ variant : ident) ,* $ (,) ? }) => { impl $ enum_name { pub const ALL : &'static [Self] = & [$ (Self ::$ variant) ,*] ; pub fn as_str (& self) -> &'static str { match self { $ (Self ::$ variant => stringify ! ($ variant)) ,* } } pub fn visit_method_name (& self) -> String { format ! ("visit_{}" , self . as_str () . to_lowercase ()) } pub fn syn_type_name (& self) -> String { format ! ("syn::{}" , self . as_str ()) } } } ; }
    };
}

mkmeta!();