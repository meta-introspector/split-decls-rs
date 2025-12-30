// Generated macro for cli (module)
macro_rules! Depcratecli {
() => {
// Module: crate
// Provides: {"cli"}
// Dependencies: {}
pub mod cli { pub use super :: imports :: wasi :: cli :: * ; pub mod command { # [doc = " Generate an exported instance of the `wasi:cli/command` world."] # [doc = ""] # [doc = " This macro generate the `#[no_mangle]` functions necessary to"] # [doc = " export this interface. It takes an argument which is a type that"] # [doc = " must implement the"] # [doc = " [`exports::cli::run::Guest`](crate::exports::cli::run::Guest)"] # [doc = " trait."] # [doc = ""] # [doc = " ```"] # [doc = " struct MyCliRunner;"] # [doc = ""] # [doc = " impl wasip2::exports::cli::run::Guest for MyCliRunner {"] # [doc = "     fn run() -> Result<(), ()> {"] # [doc = "         // ..."] # [doc = " # panic!();"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " wasip2::cli::command::export!(MyCliRunner);"] # [doc = " ```"] # [doc = ""] # [doc = " ## Compatibility with `wasm32-wasip1` targets"] # [doc = ""] # [doc = " This macro is not compatible with `wasm32-wasip1` `bin` targets"] # [doc = " which instead use a `fn main()` with the"] # [doc = " `wasi_snapshot_preview1.command.wasm` adapter. This macro _can_ be"] # [doc = " used with the `reactor` or `proxy` adapters."] # [doc = ""] # [doc = " <!--"] # [doc = " The marker above hides the generated documentation by wit-bindgen for this"] # [doc = " macro."] # [doc = " -->"] # [doc (inline)] pub use crate :: command :: _export_command as export ; } }
};
}
