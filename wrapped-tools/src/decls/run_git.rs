macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! run_git {
    () => {
        deps!();
        # [doc = " Run `git` in `working_dir` with all provided `args`."] pub fn run_git (working_dir : & Path , args : & [& str]) -> std :: io :: Result < std :: process :: ExitStatus > { std :: process :: Command :: new (GIT_PROGRAM) . current_dir (working_dir) . args (args) . status () }
    };
}

run_git!();