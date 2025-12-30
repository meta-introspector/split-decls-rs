// Generated macro for format_struct (function)
macro_rules! Depcrate_itemsformat_struct {
() => {
// Module: crate::items
// Provides: {"format_struct"}
// Dependencies: {}
fn format_struct (context : & RewriteContext < '_ > , struct_parts : & StructParts < '_ > , offset : Indent , one_line_width : Option < usize > ,) -> Option < String > { match struct_parts . def { ast :: VariantData :: Unit (..) => format_unit_struct (context , struct_parts , offset) , ast :: VariantData :: Tuple (fields , _) => { format_tuple_struct (context , struct_parts , fields , offset) } ast :: VariantData :: Struct { fields , .. } => { format_struct_struct (context , struct_parts , fields , offset , one_line_width) } } }
};
}
