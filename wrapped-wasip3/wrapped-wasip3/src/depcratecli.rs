// Generated macro for cli (module)
macro_rules! Depcratecli {
() => {
// Module: crate
// Provides: {"cli"}
// Dependencies: {}
pub mod cli { pub use super :: imports :: wasi :: cli :: * ; pub mod command { # [doc = " Generate an exported instance of the `wasi:cli/command` world."] # [doc = ""] # [doc = " This macro generate the `#[no_mangle]` functions necessary to"] # [doc = " export this interface. It takes an argument which is a type that"] # [doc = " must implement the"] # [doc = " [`exports::cli::run::Guest`](crate::exports::cli::run::Guest)"] # [doc = " trait."] # [doc = ""] # [doc = " ```"] # [doc = " struct MyCliRunner;"] # [doc = ""] # [doc = " impl wasip3::exports::cli::run::Guest for MyCliRunner {"] # [doc = "     async fn run() -> Result<(), ()> {"] # [doc = "         // ..."] # [doc = " # panic!();"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " wasip3::cli::command::export!(MyCliRunner);"] # [doc = " ```"] # [doc = ""] # [doc = " ## Incompatibility with `bin` target"] # [doc = ""] # [doc = " This macro is not compatible with the Rust `bin` crate target"] # [doc = " which instead use a `fn main()`. This macro can, however, be used"] # [doc = " with the `cdylib` crate target."] # [doc = ""] # [doc = " <!--"] # [doc = " The marker above hides the generated documentation by wit-bindgen for this"] # [doc = " macro."] # [doc (inline)] pub use crate :: command :: _export_command as export ; } }
};
}
