// Generated macro for impl_919 (impl)
macro_rules! Depcrate_compiler_rustimpl_919 {
() => {
// Module: crate::compiler::rust
// Provides: {"impl_919"}
// Dependencies: {}
# [cfg (feature = "dist-client")] # [cfg (all (target_os = "linux" , target_arch = "x86_64"))] impl pkg :: ToolchainPackager for RustToolchainPackager { fn write_pkg (self : Box < Self > , f : fs :: File) -> Result < () > { info ! ("Packaging Rust compiler for sysroot {}" , self . sysroot . display ()) ; let RustToolchainPackager { sysroot } = * self ; let mut package_builder = pkg :: ToolchainPackageBuilder :: new () ; package_builder . add_common () ? ; let bins_path = sysroot . join (BINS_DIR) ; let sysroot_executable = bins_path . join ("rustc") . with_extension (EXE_EXTENSION) ; package_builder . add_executable_and_deps (sysroot_executable) ? ; package_builder . add_dir_contents (& bins_path) ? ; if BINS_DIR != LIBS_DIR { let libs_path = sysroot . join (LIBS_DIR) ; package_builder . add_dir_contents (& libs_path) ? } package_builder . into_compressed_tar (f) } }
};
}
