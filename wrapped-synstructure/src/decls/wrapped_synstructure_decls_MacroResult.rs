use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Helper trait describing values which may be returned by macro implementation
/// methods used by this crate's macros.
pub trait MacroResult {
    /// Convert this result into a `Result` for further processing / validation.
    fn into_result(self) -> Result<TokenStream>;
    /// Convert this result into a `proc_macro::TokenStream`, ready to return
    /// from a native `proc_macro` implementation.
    ///
    /// If `into_result()` would return an `Err`, this method should instead
    /// generate a `compile_error!` invocation to nicely report the error.
    ///
    /// *This method is available if `synstructure` is built with the
    /// `"proc-macro"` feature.*
    #[cfg(
        all(
            not(
                all(
                    target_arch = "wasm32",
                    any(target_os = "unknown", target_os = "wasi")
                )
            ),
            feature = "proc-macro"
        )
    )]
    fn into_stream(self) -> proc_macro::TokenStream
    where
        Self: Sized,
    {
        match self.into_result() {
            Ok(ts) => ts.into(),
            Err(err) => err.to_compile_error().into(),
        }
    }
}
