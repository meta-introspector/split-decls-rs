use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub (crate) fn current_version (_input : TokenStream) -> TokenStream { let env_var = "CFG_RELEASE" ; TokenStream :: from (match RustcVersion :: parse_cfg_release (env_var) { Ok (RustcVersion { major , minor , patch }) => quote ! (Self { major : # major , minor : # minor , patch : # patch }) , Err (err) => syn :: Error :: new (Span :: call_site () , format ! ("{env_var} env var: {err}")) . into_compile_error () , }) }
}