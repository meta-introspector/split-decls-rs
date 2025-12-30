// Generated macro for group_by (function)
macro_rules! Depcrate_builtins_filters_arraygroup_by {
() => {
// Module: crate::builtins::filters::array
// Provides: {"group_by"}
// Dependencies: {}
# [doc = " Group the array values by the `attribute` given"] # [doc = " Returns a hashmap of key => values, items without the `attribute` or where `attribute` is `null` are discarded."] # [doc = " The returned keys are stringified"] pub fn group_by (value : & Value , args : & HashMap < String , Value >) -> Result < Value > { let arr = try_get_value ! ("group_by" , "value" , Vec < Value >, value) ; if arr . is_empty () { return Ok (Map :: new () . into ()) ; } let key = match args . get ("attribute") { Some (val) => try_get_value ! ("group_by" , "attribute" , String , val) , None => { return Err (Error :: msg ("The `group_by` filter has to have an `attribute` argument")) } } ; let mut grouped = Map :: new () ; for val in arr { if let Some (key_val) = dotted_pointer (& val , & key) . cloned () { if key_val . is_null () { continue ; } let str_key = match key_val . as_str () { Some (key) => key . to_owned () , None => format ! ("{}" , key_val) , } ; if let Some (vals) = grouped . get_mut (& str_key) { vals . as_array_mut () . unwrap () . push (val) ; continue ; } grouped . insert (str_key , Value :: Array (vec ! [val])) ; } } Ok (to_value (grouped) . unwrap ()) }
};
}
