// Generated macro for build_qemu_run (function)
macro_rules! Depcrate_backcompatbuild_qemu_run {
() => {
// Module: crate::backcompat
// Provides: {"build_qemu_run"}
// Dependencies: {}
fn build_qemu_run (tempdir : & Path) -> anyhow :: Result < PathBuf > { run_silently (Command :: new ("cargo") . args (["build" , "-p" , "qemu-run"]) . current_dir (tempdir) , | | anyhow ! ("`cargo build` failed") ,) ? ; let mut executable_path = tempdir . to_owned () ; executable_path . push ("target") ; executable_path . push ("debug") ; executable_path . push ("qemu-run") ; assert ! (executable_path . exists () , "`qemu-run` executable not found") ; Ok (executable_path) }
};
}
