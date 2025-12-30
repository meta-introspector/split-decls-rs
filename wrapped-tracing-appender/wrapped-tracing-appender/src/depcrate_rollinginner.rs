// Generated macro for Inner (struct)
macro_rules! Depcrate_rollingInner {
() => {
// Module: crate::rolling
// Provides: {"Inner"}
// Dependencies: {}
# [derive (Debug)] struct Inner { log_directory : PathBuf , log_filename_prefix : Option < String > , log_filename_suffix : Option < String > , date_format : Vec < format_description :: FormatItem < 'static > > , rotation : Rotation , next_date : AtomicUsize , max_files : Option < usize > , }
};
}
