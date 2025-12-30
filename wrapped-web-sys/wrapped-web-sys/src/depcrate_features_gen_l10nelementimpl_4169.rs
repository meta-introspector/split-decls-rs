// Generated macro for impl_4169 (impl)
macro_rules! Depcrate_features_gen_L10nElementimpl_4169 {
() => {
// Module: crate::features::gen_L10nElement
// Provides: {"impl_4169"}
// Dependencies: {}
impl L10nElement { # [doc = "Construct a new `L10nElement`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `L10nElement`*"] pub fn new (l10n_id : & str , local_name : & str , namespace_uri : & str) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_l10n_id (l10n_id) ; ret . set_local_name (local_name) ; ret . set_namespace_uri (namespace_uri) ; ret } # [deprecated = "Use `set_l10n_args()` instead."] pub fn l10n_args (& mut self , val : Option < & :: js_sys :: Object >) -> & mut Self { self . set_l10n_args (val) ; self } # [deprecated = "Use `set_l10n_attrs()` instead."] pub fn l10n_attrs (& mut self , val : Option < & str >) -> & mut Self { self . set_l10n_attrs (val) ; self } # [deprecated = "Use `set_l10n_id()` instead."] pub fn l10n_id (& mut self , val : & str) -> & mut Self { self . set_l10n_id (val) ; self } # [deprecated = "Use `set_local_name()` instead."] pub fn local_name (& mut self , val : & str) -> & mut Self { self . set_local_name (val) ; self } # [deprecated = "Use `set_namespace_uri()` instead."] pub fn namespace_uri (& mut self , val : & str) -> & mut Self { self . set_namespace_uri (val) ; self } # [deprecated = "Use `set_type()` instead."] pub fn type_ (& mut self , val : Option < & str >) -> & mut Self { self . set_type (val) ; self } }
};
}
