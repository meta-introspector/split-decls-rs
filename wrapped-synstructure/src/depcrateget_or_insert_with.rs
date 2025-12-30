// Generated macro for get_or_insert_with (function)
macro_rules! Depcrateget_or_insert_with {
() => {
// Module: crate
// Provides: {"get_or_insert_with"}
// Dependencies: {}
# [doc = " Helper method which does the same thing as rustc 1.20's"] # [doc = " `Option::get_or_insert_with`. This method is used to keep backwards"] # [doc = " compatibility with rustc 1.15."] fn get_or_insert_with < T , F > (opt : & mut Option < T > , f : F) -> & mut T where F : FnOnce () -> T , { if opt . is_none () { * opt = Some (f ()) ; } match opt { Some (v) => v , None => unreachable ! () , } }
};
}
