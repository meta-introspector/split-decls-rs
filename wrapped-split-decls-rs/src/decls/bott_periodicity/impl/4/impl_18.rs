use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl BottMacroGenerator { pub fn new (input : TokenStream) -> Self { let base_bundle = AbstractionBundle { bott_level : BottLevel :: Zero , winding_number : 0 , content : AbstractionContent :: Concrete (input) , chern_classes : vec ! [0] , } ; let mut tower = SuspensionTower :: new (base_bundle) ; tower . build_full_period () ; Self { tower } } pub fn generate_all_levels (& self) -> TokenStream { let mut output = TokenStream :: new () ; for level in 0 .. 8 { let macro_code = self . tower . generate_macro_at_level (level) ; output . extend (quote ! { # macro_code }) ; } output } }
}