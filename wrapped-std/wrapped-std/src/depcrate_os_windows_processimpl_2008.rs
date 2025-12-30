// Generated macro for impl_2008 (impl)
macro_rules! Depcrate_os_windows_processimpl_2008 {
() => {
// Module: crate::os::windows::process
// Provides: {"impl_2008"}
// Dependencies: {}
# [stable (feature = "windows_process_extensions" , since = "1.16.0")] impl CommandExt for process :: Command { fn creation_flags (& mut self , flags : u32) -> & mut process :: Command { self . as_inner_mut () . creation_flags (flags) ; self } fn show_window (& mut self , cmd_show : u16) -> & mut process :: Command { self . as_inner_mut () . show_window (Some (cmd_show)) ; self } fn force_quotes (& mut self , enabled : bool) -> & mut process :: Command { self . as_inner_mut () . force_quotes (enabled) ; self } fn raw_arg < S : AsRef < OsStr > > (& mut self , raw_text : S) -> & mut process :: Command { self . as_inner_mut () . raw_arg (raw_text . as_ref ()) ; self } fn async_pipes (& mut self , always_async : bool) -> & mut process :: Command { let _ = always_async ; self } fn spawn_with_attributes (& mut self , attribute_list : & ProcThreadAttributeList < '_ > ,) -> io :: Result < process :: Child > { self . as_inner_mut () . spawn_with_attributes (sys :: process :: Stdio :: Inherit , true , Some (attribute_list)) . map (process :: Child :: from_inner) } fn startupinfo_fullscreen (& mut self , enabled : bool) -> & mut process :: Command { self . as_inner_mut () . startupinfo_fullscreen (enabled) ; self } fn startupinfo_untrusted_source (& mut self , enabled : bool) -> & mut process :: Command { self . as_inner_mut () . startupinfo_untrusted_source (enabled) ; self } fn startupinfo_force_feedback (& mut self , enabled : Option < bool >) -> & mut process :: Command { self . as_inner_mut () . startupinfo_force_feedback (enabled) ; self } }
};
}
