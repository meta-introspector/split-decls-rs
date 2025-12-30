// Generated macro for impl_116 (impl)
macro_rules! Depcrate_attrimpl_116 {
() => {
// Module: crate::attr
// Provides: {"impl_116"}
// Dependencies: {}
impl < 'a > AttrCtxt < 'a > { pub fn new (inner : & 'a Ctxt , attrs : & 'a [Attribute] , attr_type : & 'static str) -> Self { Self { inner , attr_type , metas : get_meta_items (attrs , attr_type , inner) , } } pub fn new_nested_meta (& self , metas : Vec < CustomMeta >) -> Self { Self { metas , .. * self } } pub fn parse_meta (& mut self , mut handle : impl FnMut (CustomMeta , & str , & Self) -> Result < () , CustomMeta > ,) { let metas = std :: mem :: take (& mut self . metas) ; self . metas = metas . into_iter () . filter_map (| meta | match meta . path () . get_ident () . map (Ident :: to_string) { Some (ident) => handle (meta , & ident , self) . err () , _ => Some (meta) , }) . collect () ; } pub fn error_spanned_by < A : ToTokens , T : std :: fmt :: Display > (& self , obj : A , msg : T) { self . inner . error_spanned_by (obj , msg) ; } pub fn syn_error (& self , err : syn :: Error) { self . inner . syn_error (err) ; } pub fn mutual_exclusive_error (& self , meta : & CustomMeta , other_attr : & str) { if self . attr_type == "schemars" { self . error_spanned_by (meta , format_args ! ("schemars attribute cannot contain both `{}` and `{}`" , path_str (meta . path ()) , other_attr ,) ,) ; } } pub fn duplicate_error (& self , meta : & CustomMeta) { if self . attr_type == "schemars" { self . error_spanned_by (meta , format_args ! ("duplicate schemars attribute item `{}`" , path_str (meta . path ())) ,) ; } } }
};
}
