// Generated macro for check_license_exceptions (function)
macro_rules! Depcrate_depscheck_license_exceptions {
() => {
// Module: crate::deps
// Provides: {"check_license_exceptions"}
// Dependencies: {}
# [doc = " Check that all licenses of tool dependencies are in the valid list in `LICENSES`."] # [doc = ""] # [doc = " Packages listed in `exceptions` are allowed for tools."] fn check_license_exceptions (metadata : & Metadata , workspace : & str , exceptions : & [(& str , & str)] , bad : & mut bool ,) { for (name , license) in exceptions { if ! metadata . packages . iter () . any (| p | * p . name == * name) { tidy_error ! (bad , "could not find exception package `{}` in workspace `{workspace}`\n\
                Remove from EXCEPTIONS list if it is no longer used." , name) ; } for pkg in metadata . packages . iter () . filter (| p | * p . name == * name) { match & pkg . license { None => { tidy_error ! (bad , "dependency exception `{}` in workspace `{workspace}` does not declare a license expression" , pkg . id) ; } Some (pkg_license) => { if pkg_license . as_str () != * license { println ! ("dependency exception `{name}` license in workspace `{workspace}` has changed") ; println ! ("    previously `{license}` now `{pkg_license}`") ; println ! ("    update EXCEPTIONS for the new license") ; * bad = true ; } } } } } let exception_names : Vec < _ > = exceptions . iter () . map (| (name , _license) | * name) . collect () ; for pkg in & metadata . packages { if pkg . source . is_none () { continue ; } if exception_names . contains (& pkg . name . as_str ()) { continue ; } let license = match & pkg . license { Some (license) => license , None => { tidy_error ! (bad , "dependency `{}` in workspace `{workspace}` does not define a license expression" , pkg . id) ; continue ; } } ; if ! LICENSES . contains (& license . as_str ()) { tidy_error ! (bad , "invalid license `{}` for package `{}` in workspace `{workspace}`" , license , pkg . id) ; } } }
};
}
