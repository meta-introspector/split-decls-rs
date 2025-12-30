// Generated macro for deserialize_field_identifier (function)
macro_rules! Depcrate_dedeserialize_field_identifier {
() => {
// Module: crate::de
// Provides: {"deserialize_field_identifier"}
// Dependencies: {}
# [doc = " Generates enum and its `Deserialize` implementation that represents each"] # [doc = " non-skipped field of the struct"] fn deserialize_field_identifier (deserialized_fields : & [FieldWithAliases] , cattrs : & attr :: Container , has_flatten : bool ,) -> Stmts { let (ignore_variant , fallthrough) = if has_flatten { let ignore_variant = quote ! (__other (_serde :: __private :: de :: Content <'de >) ,) ; let fallthrough = quote ! (_serde :: __private :: Ok (__Field :: __other (__value))) ; (Some (ignore_variant) , Some (fallthrough)) } else if cattrs . deny_unknown_fields () { (None , None) } else { let ignore_variant = quote ! (__ignore ,) ; let fallthrough = quote ! (_serde :: __private :: Ok (__Field :: __ignore)) ; (Some (ignore_variant) , Some (fallthrough)) } ; Stmts (deserialize_generated_identifier (deserialized_fields , has_flatten , false , ignore_variant , fallthrough ,)) }
};
}
