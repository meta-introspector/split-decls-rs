// Generated macro for update_env (function)
macro_rules! Depcrateupdate_env {
() => {
// Module: crate
// Provides: {"update_env"}
// Dependencies: {}
fn update_env < K , V > (key : K , value : Option < V >) where K : AsRef < OsStr > , V : AsRef < OsStr > , { match value { Some (v) => env :: set_var (key , v) , None => env :: remove_var (key) , } }
};
}
