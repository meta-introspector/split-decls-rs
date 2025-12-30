// Generated macro for map (function)
macro_rules! Depcrate_builtins_filters_arraymap {
() => {
// Module: crate::builtins::filters::array
// Provides: {"map"}
// Dependencies: {}
# [doc = " Map retrieves an attribute from a list of objects."] # [doc = " The 'attribute' argument specifies what to retrieve."] pub fn map (value : & Value , args : & HashMap < String , Value >) -> Result < Value > { let arr = try_get_value ! ("map" , "value" , Vec < Value >, value) ; if arr . is_empty () { return Ok (arr . into ()) ; } let attribute = match args . get ("attribute") { Some (val) => try_get_value ! ("map" , "attribute" , String , val) , None => return Err (Error :: msg ("The `map` filter has to have an `attribute` argument")) , } ; let arr = arr . into_iter () . filter_map (| v | match dotted_pointer (& v , & attribute) { Some (val) if ! val . is_null () => Some (val . clone ()) , _ => None , }) . collect :: < Vec < _ > > () ; Ok (to_value (arr) . unwrap ()) }
};
}
