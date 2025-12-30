// Generated macro for add_link_args (function)
macro_rules! Depcrate_specadd_link_args {
() => {
// Module: crate::spec
// Provides: {"add_link_args"}
// Dependencies: {}
fn add_link_args (link_args : & mut LinkArgs , flavor : LinkerFlavor , args : & [& 'static str]) { add_link_args_iter (link_args , flavor , args . iter () . copied () . map (Cow :: Borrowed)) }
};
}
