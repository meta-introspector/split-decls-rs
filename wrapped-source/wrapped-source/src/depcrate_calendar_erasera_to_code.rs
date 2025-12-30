// Generated macro for era_to_code (function)
macro_rules! Depcrate_calendar_erasera_to_code {
() => {
// Module: crate::calendar::eras
// Provides: {"era_to_code"}
// Dependencies: {}
# [doc = " See <https://docs.google.com/document/d/1vMVhMHgCYRyx2gmwEfKRyXWDg_lrQadd8iMVU9uPK1o/edit?usp=chrome_omnibox&ouid=111665445991279316689>"] # [doc = " for the era identifier spec"] pub (crate) fn era_to_code (original : & str , year : i32) -> String { let name = original . split (' ') . next () . expect ("split iterator is non-empty") ; let name = name . replace (['ō' , 'Ō'] , "o") . replace (['ū' , 'Ū'] , "u") . replace (['-' , '\'' , '’'] , "") . to_lowercase () ; if ! name . is_ascii () { panic ! ("Era name {name} (parsed from {original}) contains non-ascii characters") ; } format ! ("{name}-{year}") }
};
}
