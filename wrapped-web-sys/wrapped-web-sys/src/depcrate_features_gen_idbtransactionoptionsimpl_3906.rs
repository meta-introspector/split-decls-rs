// Generated macro for impl_3906 (impl)
macro_rules! Depcrate_features_gen_IdbTransactionOptionsimpl_3906 {
() => {
// Module: crate::features::gen_IdbTransactionOptions
// Provides: {"impl_3906"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl IdbTransactionOptions { # [doc = "Construct a new `IdbTransactionOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `IdbTransactionOptions`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "IdbTransactionDurability")] # [deprecated = "Use `set_durability()` instead."] pub fn durability (& mut self , val : IdbTransactionDurability) -> & mut Self { self . set_durability (val) ; self } }
};
}
