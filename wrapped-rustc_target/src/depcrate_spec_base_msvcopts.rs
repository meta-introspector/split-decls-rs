// Generated macro for opts (function)
macro_rules! Depcrate_spec_base_msvcopts {
() => {
// Module: crate::spec::base::msvc
// Provides: {"opts"}
// Dependencies: {}
pub (crate) fn opts () -> TargetOptions { let pre_link_args = TargetOptions :: link_args (LinkerFlavor :: Msvc (Lld :: No) , & ["/NOLOGO"]) ; TargetOptions { linker_flavor : LinkerFlavor :: Msvc (Lld :: No) , dll_tls_export : false , is_like_windows : true , is_like_msvc : true , binary_format : BinaryFormat :: Coff , pre_link_args , abi_return_struct_as_int : true , emit_debug_gdb_scripts : false , archive_format : "coff" . into () , split_debuginfo : SplitDebuginfo :: Packed , supported_split_debuginfo : Cow :: Borrowed (& [SplitDebuginfo :: Packed]) , debuginfo_kind : DebuginfoKind :: Pdb , .. Default :: default () } }
};
}
