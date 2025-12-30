// Generated macro for test (module)
macro_rules! Depcrate_datetimetest {
() => {
// Module: crate::datetime
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use icu :: { datetime :: provider :: skeleton :: reference :: Skeleton , locale :: langid , plurals :: PluralElements , } ; # [test] # [ignore] fn test_datetime_skeletons () { let skeletons = SourceDataProvider :: new_testing () . get_dates_resource (& langid ! ("fil") . into () , Some (DatagenCalendar :: Gregorian)) . unwrap () . datetime_formats . available_formats . parse_skeletons () ; assert_eq ! (Some (& PluralElements :: new ("L" . parse () . expect ("Failed to create pattern"))) , skeletons . get (& Skeleton :: try_from ("M") . expect ("Failed to create Skeleton"))) ; let expected = PluralElements :: new ("'linggo' w 'ng' Y" . parse () . expect ("Failed to create pattern") ,) . with_one_value (Some ("'ika'-w 'linggo' 'ng' Y" . parse () . expect ("Failed to create pattern") ,)) ; assert_eq ! (Some (& expected) , skeletons . get (& Skeleton :: try_from ("yw") . expect ("Failed to create Skeleton"))) ; } }
};
}
