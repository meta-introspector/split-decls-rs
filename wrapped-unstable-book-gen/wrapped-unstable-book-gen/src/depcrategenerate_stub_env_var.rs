// Generated macro for generate_stub_env_var (function)
macro_rules! Depcrategenerate_stub_env_var {
() => {
// Module: crate
// Provides: {"generate_stub_env_var"}
// Dependencies: {}
fn generate_stub_env_var (path : & Path , name : & str) { let content = format ! (include_str ! ("stub-env-var.md") , name = name) ; t ! (write (path , content) , path) ; }
};
}
