// Generated macro for json_field_to_formatting_attribute (function)
macro_rules! Depcrate_personnames_person_names_format_data_providersjson_field_to_formatting_attribute {
() => {
// Module: crate::personnames::person_names_format_data_providers
// Provides: {"json_field_to_formatting_attribute"}
// Dependencies: {}
fn json_field_to_formatting_attribute (value : & str) -> Result < u32 , DataError > { match value { "surnameFirst" => Ok (PersonNamesFormattingAttributes :: SurnameFirst . bit_value ()) , "givenFirst" => Ok (PersonNamesFormattingAttributes :: GivenFirst . bit_value ()) , "sorting" => Ok (PersonNamesFormattingAttributes :: Sorting . bit_value ()) , "long" => Ok (PersonNamesFormattingAttributes :: Long . bit_value ()) , "medium" => Ok (PersonNamesFormattingAttributes :: Medium . bit_value ()) , "short" => Ok (PersonNamesFormattingAttributes :: Short . bit_value ()) , "addressing" => Ok (PersonNamesFormattingAttributes :: Addressing . bit_value ()) , "referring" => Ok (PersonNamesFormattingAttributes :: Referring . bit_value ()) , "monogram" => Ok (PersonNamesFormattingAttributes :: Monogram . bit_value ()) , v if v . starts_with ("formal") => Ok (PersonNamesFormattingAttributes :: Formal . bit_value ()) , v if v . starts_with ("informal") => Ok (PersonNamesFormattingAttributes :: Informal . bit_value ()) , _ => Err (DataError :: custom ("invalid json value")) , } }
};
}
