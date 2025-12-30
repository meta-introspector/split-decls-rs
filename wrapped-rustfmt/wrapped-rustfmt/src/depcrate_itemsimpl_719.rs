// Generated macro for impl_719 (impl)
macro_rules! Depcrate_itemsimpl_719 {
() => {
// Module: crate::items
// Provides: {"impl_719"}
// Dependencies: {}
impl < 'a > StructParts < 'a > { fn format_header (& self , context : & RewriteContext < '_ > , offset : Indent) -> String { format_header (context , self . prefix , self . ident , self . vis , offset) } fn from_variant (variant : & 'a ast :: Variant , context : & RewriteContext < '_ >) -> Self { StructParts { prefix : "" , ident : variant . ident , vis : & DEFAULT_VISIBILITY , def : & variant . data , generics : None , span : enum_variant_span (variant , context) , } } pub (crate) fn from_item (item : & 'a ast :: Item) -> Self { let (prefix , def , ident , generics) = match item . kind { ast :: ItemKind :: Struct (ident , ref def , ref generics) => { ("struct " , def , ident , generics) } ast :: ItemKind :: Union (ident , ref def , ref generics) => ("union " , def , ident , generics) , _ => unreachable ! () , } ; StructParts { prefix , ident , vis : & item . vis , def , generics : Some (generics) , span : item . span , } } }
};
}
