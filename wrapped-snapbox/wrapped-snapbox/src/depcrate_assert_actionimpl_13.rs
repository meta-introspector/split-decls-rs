// Generated macro for impl_13 (impl)
macro_rules! Depcrate_assert_actionimpl_13 {
() => {
// Module: crate::assert::action
// Provides: {"impl_13"}
// Dependencies: {}
impl Action { pub fn with_env_var (var : impl AsRef < std :: ffi :: OsStr >) -> Option < Self > { let var = var . as_ref () ; let value = std :: env :: var_os (var) ? ; Self :: with_env_value (value) } pub fn with_env_value (value : impl AsRef < std :: ffi :: OsStr >) -> Option < Self > { let value = value . as_ref () ; match value . to_str () ? { "skip" => Some (Action :: Skip) , "ignore" => Some (Action :: Ignore) , "verify" => Some (Action :: Verify) , "overwrite" => Some (Action :: Overwrite) , _ => None , } } }
};
}
