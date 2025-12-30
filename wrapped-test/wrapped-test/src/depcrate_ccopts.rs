// Generated macro for COpts (struct)
macro_rules! Depcrate_cCOpts {
() => {
// Module: crate::c
// Provides: {"COpts"}
// Dependencies: {}
# [derive (Default , Debug , Clone , Parser)] pub struct COpts { # [doc = " Path to the installation of wasi-sdk"] # [clap (long , env = "WASI_SDK_PATH" , value_name = "PATH")] pub (crate) wasi_sdk_path : Option < PathBuf > , # [doc = " Name of the C target to compile for."] # [clap (long , default_value = "wasm32-wasip2" , value_name = "TARGET")] c_target : String , }
};
}
