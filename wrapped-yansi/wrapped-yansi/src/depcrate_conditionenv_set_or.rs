// Generated macro for env_set_or (function)
macro_rules! Depcrate_conditionenv_set_or {
() => {
// Module: crate::condition
// Provides: {"env_set_or"}
// Dependencies: {}
# [cfg (feature = "detect-env")] pub fn env_set_or (name : & str , default : bool) -> bool { std :: env :: var_os (name) . map_or (default , | v | v != "0") }
};
}
