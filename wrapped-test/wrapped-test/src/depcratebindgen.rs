// Generated macro for Bindgen (struct)
macro_rules! DepcrateBindgen {
() => {
// Module: crate
// Provides: {"Bindgen"}
// Dependencies: {}
# [derive (Clone)] struct Bindgen { # [doc = " The arguments to the bindings generator that this component will be"] # [doc = " using."] args : Vec < String > , # [doc = " The path to the `*.wit` file or files that are having bindings"] # [doc = " generated."] wit_path : PathBuf , # [doc = " The name of the world within `wit_path` that's having bindings generated"] # [doc = " for it."] world : String , # [doc = " Configuration found in `wit_path`"] wit_config : config :: WitConfig , }
};
}
