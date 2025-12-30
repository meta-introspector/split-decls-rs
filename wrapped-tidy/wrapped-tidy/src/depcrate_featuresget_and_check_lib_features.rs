// Generated macro for get_and_check_lib_features (function)
macro_rules! Depcrate_featuresget_and_check_lib_features {
() => {
// Module: crate::features
// Provides: {"get_and_check_lib_features"}
// Dependencies: {}
fn get_and_check_lib_features (base_src_path : & Path , bad : & mut bool , lang_features : & Features ,) -> Features { let mut lib_features = Features :: new () ; map_lib_features (base_src_path , & mut | res , file , line | match res { Ok ((name , f)) => { let mut check_features = | f : & Feature , list : & Features , display : & str | { if let Some (s) = list . get (name) && f . tracking_issue != s . tracking_issue && f . level != Status :: Accepted { tidy_error ! (bad , "{}:{}: feature gate {} has inconsistent `issue`: \"{}\" mismatches the {} `issue` of \"{}\"" , file . display () , line , name , f . tracking_issue_display () , display , s . tracking_issue_display () ,) ; } } ; check_features (& f , lang_features , "corresponding lang feature") ; check_features (& f , & lib_features , "previous") ; lib_features . insert (name . to_owned () , f) ; } Err (msg) => { tidy_error ! (bad , "{}:{}: {}" , file . display () , line , msg) ; } }) ; lib_features }
};
}
