macro_rules! GIT_CORE_DIR {
    () => {
        static GIT_CORE_DIR : LazyLock < PathBuf > = LazyLock :: new (| | { let output = std :: process :: Command :: new (GIT_PROGRAM) . arg ("--exec-path") . output () . expect ("can execute `git --exec-path`") ; assert ! (output . status . success () , "`git --exec-path` failed") ; output . stdout . strip_suffix (b"\n") . expect ("`git --exec-path` output to be well-formed") . to_os_str () . expect ("no invalid UTF-8 in `--exec-path` except as OS allows") . into () }) ;
    };
}

GIT_CORE_DIR!();