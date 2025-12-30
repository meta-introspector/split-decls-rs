// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "macro_4",
decl_type: "function",
source_file: "./src/config_macros.rs",
source_crate: ".",
deps: [],
uses: ["SplitDeclsConfig", "GLOBAL_CONFIG", "Mutex"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! macro_4 {
    () => {
        lazy_static ! { pub static ref GLOBAL_CONFIG : Mutex < SplitDeclsConfig > = Mutex :: new (SplitDeclsConfig :: default ()) ; }
    };
}

macro_4!();