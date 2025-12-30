// Generated macro for to_mask (function)
macro_rules! Depcrate_personnames_person_names_format_data_providersto_mask {
() => {
// Module: crate::personnames::person_names_format_data_providers
// Provides: {"to_mask"}
// Dependencies: {}
fn to_mask (ordering : & str , size : & str , referring : & str , formality : & str) -> Result < u32 , DataError > { let o = json_field_to_formatting_attribute (ordering) ? ; let s = json_field_to_formatting_attribute (size) ? ; let r = json_field_to_formatting_attribute (referring) ? ; let f = json_field_to_formatting_attribute (formality) ? ; Ok (o | s | r | f) }
};
}
