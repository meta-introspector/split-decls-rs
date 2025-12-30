// Generated macro for json_pointer_mut (function)
macro_rules! Depcrate_generatejson_pointer_mut {
() => {
// Module: crate::generate
// Provides: {"json_pointer_mut"}
// Dependencies: {}
fn json_pointer_mut < 'a > (mut object : & 'a mut JsonMap < String , Value > , pointer : & str , create_if_missing : bool ,) -> Option < & 'a mut JsonMap < String , Value > > { use serde_json :: map :: Entry ; let pointer = pointer . strip_prefix ('/') ? ; if pointer . is_empty () { return Some (object) ; } for mut segment in pointer . split ('/') { let replaced : String ; if segment . contains ('~') { replaced = segment . replace ("~1" , "/") . replace ("~0" , "~") ; segment = & replaced ; } let next_value = match object . entry (segment) { Entry :: Occupied (o) => o . into_mut () , Entry :: Vacant (v) if create_if_missing => v . insert (Value :: Object (JsonMap :: new ())) , Entry :: Vacant (_) => return None , } ; object = next_value . as_object_mut () ? ; } Some (object) }
};
}
