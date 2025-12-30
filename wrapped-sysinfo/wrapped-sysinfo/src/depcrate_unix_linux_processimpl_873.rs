// Generated macro for impl_873 (impl)
macro_rules! Depcrate_unix_linux_processimpl_873 {
() => {
// Module: crate::unix::linux::process
// Provides: {"impl_873"}
// Dependencies: {}
impl FileCounter { fn new (f : File) -> Option < Self > { let any_remaining = remaining_files () . fetch_update (Ordering :: SeqCst , Ordering :: SeqCst , | remaining | { if remaining > 0 { Some (remaining - 1) } else { None } }) ; any_remaining . ok () . map (| _ | Self (f)) } }
};
}
