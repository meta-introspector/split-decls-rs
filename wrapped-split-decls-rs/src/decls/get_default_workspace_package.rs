// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "get_default_workspace_package",
decl_type: "function",
source_file: "./src/workspace_manager.rs",
source_crate: ".",
deps: [],
uses: ["Table", "Boolean", "String", "Value", "AGPL", "Array"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! get_default_workspace_package {
    () => {
        fn get_default_workspace_package () -> toml :: Table { let mut default_package = toml :: Table :: new () ; default_package . insert ("edition" . to_string () , toml :: Value :: String ("2024" . to_string ())) ; default_package . insert ("version" . to_string () , toml :: Value :: String ("1.0.0" . to_string ())) ; default_package . insert ("publish" . to_string () , toml :: Value :: Boolean (false)) ; default_package . insert ("keywords" . to_string () , toml :: Value :: Array (vec ! [])) ; default_package . insert ("rust-version" . to_string () , toml :: Value :: String ("1.85.0" . to_string ())) ; default_package . insert ("include" . to_string () , toml :: Value :: Array (vec ! [])) ; default_package . insert ("license" . to_string () , toml :: Value :: String ("AGPL 3.0" . to_string ())) ; default_package . insert ("authors" . to_string () , toml :: Value :: Array (vec ! [])) ; default_package . insert ("description" . to_string () , toml :: Value :: String ("" . to_string ())) ; default_package . insert ("categories" . to_string () , toml :: Value :: Array (vec ! [])) ; default_package . insert ("repository" . to_string () , toml :: Value :: String ("" . to_string ())) ; default_package . insert ("homepage" . to_string () , toml :: Value :: String ("" . to_string ())) ; default_package }
    };
}

get_default_workspace_package!();