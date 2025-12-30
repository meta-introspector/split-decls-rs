// Generated macro for number_from_env_var (function)
macro_rules! Depcrate_confignumber_from_env_var {
() => {
// Module: crate::config
// Provides: {"number_from_env_var"}
// Dependencies: {}
fn number_from_env_var < A : std :: str :: FromStr > (env_var_name : & str) -> Option < Result < A > > where < A as FromStr > :: Err : std :: fmt :: Debug , { let value = env :: var (env_var_name) . ok () ? ; value . parse :: < A > () . map_err (| err | anyhow ! ("{env_var_name} value is invalid: {err:?}")) . into () }
};
}
