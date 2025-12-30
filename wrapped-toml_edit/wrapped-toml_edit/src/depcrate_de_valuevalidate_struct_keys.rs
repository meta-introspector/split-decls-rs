// Generated macro for validate_struct_keys (function)
macro_rules! Depcrate_de_valuevalidate_struct_keys {
() => {
// Module: crate::de::value
// Provides: {"validate_struct_keys"}
// Dependencies: {}
pub (crate) fn validate_struct_keys (table : & crate :: table :: KeyValuePairs , fields : & 'static [& 'static str] ,) -> Result < () , Error > { let extra_fields = table . keys () . filter_map (| key | { if ! fields . contains (& key . get ()) { Some (key . clone ()) } else { None } }) . collect :: < Vec < _ > > () ; if extra_fields . is_empty () { Ok (()) } else { Err (Error :: custom (format ! ("unexpected keys in table: {}, available keys: {}" , extra_fields . iter () . map (| k | k . get ()) . collect ::< Vec < _ >> () . join (", ") , fields . join (", ") ,) , extra_fields [0] . span () ,)) } }
};
}
