// Generated macro for impl_12 (impl)
macro_rules! Depcrate_backcompatimpl_12 {
() => {
// Module: crate::backcompat
// Provides: {"impl_12"}
// Dependencies: {}
impl QemuRun { fn build () -> anyhow :: Result < Self > { let tempdir = tempfile :: tempdir () ? ; let tempdir_path = tempdir . path () ; clone_repo (tempdir_path) ? ; let executable_path = build_qemu_run (tempdir_path) ? ; Ok (Self { executable_path , _tempdir : tempdir , }) } fn run_snapshot (& self , name : & str , feature : & str) -> anyhow :: Result < () > { println ! ("{}" , name . bold ()) ; let is_test = name . contains ("test") ; let command = if is_test { "tt" } else { "rb" } ; run_silently (Command :: new ("cargo") . args (["-q" , command , name]) . args (["--features" , feature]) . current_dir (SNAPSHOT_TESTS_DIRECTORY) . env (RUNNER_ENV_VAR , self . path ()) , | | anyhow ! ("{}" , name) ,) ? ; Ok (()) } fn path (& self) -> & Path { & self . executable_path } }
};
}
