// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "impl_378",
decl_type: "function",
source_file: "./src/goal_parser.rs",
source_crate: ".",
deps: ["Workflow"],
uses: ["Path", "Workflow", "Result", "Ok"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        Workflow!();
    };
}

macro_rules! impl_378 {
    () => {
        deps!();
        impl Workflow { pub fn load_from_file (path : & std :: path :: Path) -> anyhow :: Result < Self > { let content = std :: fs :: read_to_string (path) ? ; let workflow : Workflow = toml :: from_str (& content) ? ; Ok (workflow) } }
    };
}

impl_378!();