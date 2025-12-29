// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "impl_380",
decl_type: "function",
source_file: "./src/goal_parser.rs",
source_crate: ".",
deps: ["GoalConfig"],
uses: ["Ok", "Result", "Path", "GoalConfig"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        GoalConfig!();
    };
}

macro_rules! impl_380 {
    () => {
        deps!();
        impl GoalConfig { pub fn load_from_file (path : & std :: path :: Path) -> anyhow :: Result < Self > { let content = std :: fs :: read_to_string (path) ? ; let config : GoalConfig = toml :: from_str (& content) ? ; Ok (config) } }
    };
}

impl_380!();