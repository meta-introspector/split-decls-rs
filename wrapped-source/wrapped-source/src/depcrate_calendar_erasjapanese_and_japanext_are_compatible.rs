// Generated macro for japanese_and_japanext_are_compatible (function)
macro_rules! Depcrate_calendar_erasjapanese_and_japanext_are_compatible {
() => {
// Module: crate::calendar::eras
// Provides: {"japanese_and_japanext_are_compatible"}
// Dependencies: {}
# [test] pub fn japanese_and_japanext_are_compatible () { let provider = SourceDataProvider :: new_testing () ; let japanese = & provider . all_eras () . unwrap () [& DatagenCalendar :: JapaneseModern] ; let japanext = & provider . all_eras () . unwrap () [& DatagenCalendar :: JapaneseExtended] ; assert_eq ! (japanext . iter () . take (2) . zip (japanese . iter () . take (2)) . find (| (e , a) | e != a) , None ,) ; assert_eq ! (japanext . iter () . skip (2) . rev () . zip (japanese . iter () . skip (2) . rev ()) . find (| (e , a) | e != a) , None , "{japanext:?} - {japanese:?}") ; }
};
}
