// Generated macro for related_categories (function)
macro_rules! Depcrate_general_categoryrelated_categories {
() => {
// Module: crate::general_category
// Provides: {"related_categories"}
// Dependencies: {}
# [doc = " Return all groups of \"related\" general categories."] fn related_categories (propvals : & PropertyValues ,) -> Vec < (String , Vec < String >) > { let c = | name : & str | -> String { propvals . canonical ("gc" , name) . unwrap () . to_string () } ; vec ! [(c ("Cased_Letter") , vec ! [c ("lu") , c ("ll") , c ("lt")]) , (c ("Letter") , vec ! [c ("lu") , c ("ll") , c ("lt") , c ("lm") , c ("lo")]) , (c ("Mark") , vec ! [c ("mn") , c ("mc") , c ("me")]) , (c ("Number") , vec ! [c ("nd") , c ("nl") , c ("no")]) , (c ("Punctuation") , vec ! [c ("pc") , c ("pd") , c ("ps") , c ("pe") , c ("pi") , c ("pf") , c ("po") ,] ,) , (c ("Symbol") , vec ! [c ("sm") , c ("sc") , c ("sk") , c ("so")]) , (c ("Separator") , vec ! [c ("zs") , c ("zl") , c ("zp")]) , (c ("Other") , vec ! [c ("cc") , c ("cf") , c ("cs") , c ("co") , c ("cn")]) ,] }
};
}
