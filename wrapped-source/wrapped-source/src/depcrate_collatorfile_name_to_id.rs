// Generated macro for file_name_to_id (function)
macro_rules! Depcrate_collatorfile_name_to_id {
() => {
// Module: crate::collator
// Provides: {"file_name_to_id"}
// Dependencies: {}
fn file_name_to_id (file_name : & str) -> Vec < DataIdentifierCow < 'static > > { let (mut language , mut variant) = file_name . rsplit_once ('_') . unwrap () ; if language == "root" { language = "und" ; } let mut r = vec ! [] ; let Ok (mut locale) = DataLocale :: try_from_str (& language . replace ('_' , "-")) else { return Default :: default () ; } ; if language == "zh" { locale . language = language ! ("und") ; locale . script = Some (script ! ("Hani")) ; if variant == "pinyin" { r . push (DataIdentifierCow :: from_borrowed_and_owned (Default :: default () , "und-Hans" . parse () . unwrap () ,)) ; } else if variant == "stroke" { r . push (DataIdentifierCow :: from_borrowed_and_owned (Default :: default () , "und-Hant" . parse () . unwrap () ,)) ; } } else if variant == "standard" { variant = "" ; } let marker_attributes = match variant { "traditional" => DataMarkerAttributes :: from_str_or_panic ("trad") . to_owned () , "phonebook" => DataMarkerAttributes :: from_str_or_panic ("phonebk") . to_owned () , "dictionary" => DataMarkerAttributes :: from_str_or_panic ("dict") . to_owned () , v => match DataMarkerAttributes :: try_from_str (v) { Ok (s) => s . to_owned () , _ => return r , } , } ; r . push (DataIdentifierCow :: from_owned (marker_attributes , locale)) ; r }
};
}
