// Generated macro for impl_112 (impl)
macro_rules! Depcrate_attrimpl_112 {
() => {
// Module: crate::attr
// Provides: {"impl_112"}
// Dependencies: {}
impl VariantAttrs { pub fn new (attrs : & [Attribute] , cx : & Ctxt) -> Self { let mut result = Self :: default () ; result . populate (attrs , cx) ; result } fn populate (& mut self , attrs : & [Attribute] , cx : & Ctxt) { let schemars_cx = & mut AttrCtxt :: new (cx , attrs , "schemars") ; let serde_cx = & mut AttrCtxt :: new (cx , attrs , "serde") ; self . common . populate (attrs , schemars_cx , serde_cx) ; self . process_attr (schemars_cx) ; self . process_attr (serde_cx) ; } fn process_attr (& mut self , cx : & mut AttrCtxt) { cx . parse_meta (| m , n , c | self . process_meta (m , n , c)) ; } fn process_meta (& mut self , meta : CustomMeta , meta_name : & str , cx : & AttrCtxt ,) -> Result < () , CustomMeta > { match meta_name { "with" => match self . with { Some (WithAttr :: Type (_)) => cx . duplicate_error (& meta) , Some (WithAttr :: Function (_)) => cx . mutual_exclusive_error (& meta , "schema_with") , None => self . with = parse_name_value_lit_str (meta , cx) . ok () . map (WithAttr :: Type) , } , "schema_with" if cx . attr_type == "schemars" => match self . with { Some (WithAttr :: Function (_)) => cx . duplicate_error (& meta) , Some (WithAttr :: Type (_)) => cx . mutual_exclusive_error (& meta , "with") , None => { self . with = parse_name_value_lit_str (meta , cx) . ok () . map (WithAttr :: Function) ; } } , _ => return Err (meta) , } Ok (()) } pub fn is_default (& self) -> bool { matches ! (self , Self { common , with : None , } if common . is_default ()) } }
};
}
