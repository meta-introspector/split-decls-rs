// Generated macro for run_git (function)
macro_rules! Depcraterun_git {
() => {
// Module: crate
// Provides: {"run_git"}
// Dependencies: {}
# [doc = " Run `git` in `working_dir` with all provided `args`."] pub fn run_git (working_dir : & Path , args : & [& str]) -> std :: io :: Result < std :: process :: ExitStatus > { std :: process :: Command :: new (GIT_PROGRAM) . current_dir (working_dir) . args (args) . status () }
};
}
