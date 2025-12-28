use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < DRT : OpaqueDeriveResolution + 'static > TTMacroExpander < DRT > for MacroRulesMacroExpander < DRT > { fn expand < 'cx > (& self , cx : & 'cx mut ExtCtxt < '_ , DRT > , sp : Span , input : TokenStream ,) -> MacroExpanderResult < 'cx , DRT > { unimplemented ! () } }
}