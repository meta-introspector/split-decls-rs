use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl PpAnn for & dyn rustc_hir :: intravisit :: HirTyCtxt < '_ > { fn nested (& self , state : & mut State < '_ > , nested : Nested) { match nested { Nested :: Item (id) => state . print_item (self . hir_item (id)) , Nested :: TraitItem (id) => state . print_trait_item (self . hir_trait_item (id)) , Nested :: ImplItem (id) => state . print_impl_item (self . hir_impl_item (id)) , Nested :: ForeignItem (id) => state . print_foreign_item (self . hir_foreign_item (id)) , Nested :: Body (id) => state . print_expr (self . hir_body (id) . value) , Nested :: BodyParamPat (id , i) => state . print_pat (self . hir_body (id) . params [i] . pat) , } } }
}