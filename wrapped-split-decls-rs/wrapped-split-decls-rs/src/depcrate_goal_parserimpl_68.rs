// Generated macro for impl_68 (impl)
macro_rules! Depcrate_goal_parserimpl_68 {
() => {
// Module: crate::goal_parser
// Provides: {"impl_68"}
// Dependencies: {}
impl Workflow { pub fn load_from_file (path : & std :: path :: Path) -> anyhow :: Result < Self > { let content = std :: fs :: read_to_string (path) ? ; let workflow : Workflow = toml :: from_str (& content) ? ; Ok (workflow) } }
};
}
