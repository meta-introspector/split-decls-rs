// Generated macro for impl_70 (impl)
macro_rules! Depcrate_goal_parserimpl_70 {
() => {
// Module: crate::goal_parser
// Provides: {"impl_70"}
// Dependencies: {}
impl GoalConfig { pub fn load_from_file (path : & std :: path :: Path) -> anyhow :: Result < Self > { let content = std :: fs :: read_to_string (path) ? ; let config : GoalConfig = toml :: from_str (& content) ? ; Ok (config) } }
};
}
