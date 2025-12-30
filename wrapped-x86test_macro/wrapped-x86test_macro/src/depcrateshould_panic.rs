// Generated macro for should_panic (function)
macro_rules! Depcrateshould_panic {
() => {
// Module: crate
// Provides: {"should_panic"}
// Dependencies: {}
# [doc = " Checks is ItemFn is annotated with #[should_panic]"] fn should_panic (fun : & ItemFn) -> bool { fun . attrs . iter () . find (| & attr | { attr . path . segments . iter () . find (| & path_segment | path_segment . ident == "should_panic") . is_some () }) . is_some () }
};
}
