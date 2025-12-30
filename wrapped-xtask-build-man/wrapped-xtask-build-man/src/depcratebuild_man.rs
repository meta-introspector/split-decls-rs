// Generated macro for build_man (function)
macro_rules! Depcratebuild_man {
() => {
// Module: crate
// Provides: {"build_man"}
// Dependencies: {}
# [doc = " Builds the man pages."] fn build_man (pkg_name : & str , src_paths : & [PathBuf] , outs : & [(& str , & str)] , extra_args : & [& str] ,) -> io :: Result < () > { for (format , dst_path) in outs { eprintln ! ("Start converting `{format}` for package `{pkg_name}`...") ; let mut cmd = Command :: new (std :: env ! ("CARGO")) ; cmd . args (["run" , "--package" , "mdman" , "--"]) . args (["-t" , format , "-o" , dst_path]) . args (src_paths) . args (extra_args) ; let status = cmd . status () ? ; if ! status . success () { eprintln ! ("failed to build the man pages for package `{pkg_name}`") ; eprintln ! ("failed command: `{cmd:?}`") ; process :: exit (status . code () . unwrap_or (1)) ; } } Ok (()) }
};
}
