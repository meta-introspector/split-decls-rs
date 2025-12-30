// Generated macro for id_to_file_name (function)
macro_rules! Depcrate_collatorid_to_file_name {
() => {
// Module: crate::collator
// Provides: {"id_to_file_name"}
// Dependencies: {}
fn id_to_file_name (id : DataIdentifierBorrowed) -> String { let mut s = if id . locale . is_unknown () { "root" . to_owned () } else { id . locale . write_to_string () . replace ('-' , "_") . replace ("posix" , "POSIX") } ; if s == "und_Hant" { return "zh_stroke" . into () ; } else if s == "und_Hans" { return "zh_pinyin" . into () ; } else if s == "und_Hani" { s = "zh" . into () ; } s . push ('_') ; s . push_str (match id . marker_attributes . as_str () { "" => "standard" , "trad" => "traditional" , "phonebk" => "phonebook" , "dict" => "dictionary" , extension => extension , }) ; s }
};
}
