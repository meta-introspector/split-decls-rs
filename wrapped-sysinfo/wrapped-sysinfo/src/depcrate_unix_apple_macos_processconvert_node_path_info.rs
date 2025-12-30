// Generated macro for convert_node_path_info (function)
macro_rules! Depcrate_unix_apple_macos_processconvert_node_path_info {
() => {
// Module: crate::unix::apple::macos::process
// Provides: {"convert_node_path_info"}
// Dependencies: {}
unsafe fn convert_node_path_info (node : & libc :: vnode_info_path) -> Option < PathBuf > { if node . vip_vi . vi_stat . vst_dev == 0 { return None ; } cstr_to_rust_with_size (node . vip_path . as_ptr () as _ , Some (mem :: size_of_val (& node . vip_path)) ,) . map (PathBuf :: from) }
};
}
