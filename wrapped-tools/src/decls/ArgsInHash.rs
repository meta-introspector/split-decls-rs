macro_rules! ArgsInHash {
    () => {
        # [doc = " Don't add a suffix to the archive name as `args` are platform dependent, none-deterministic,"] # [doc = " or otherwise don't influence the content of the archive."] # [doc = " Note that this also means that `args` won't be used to control the hash of the archive itself."] # [derive (Copy , Clone)] enum ArgsInHash { Yes , No , }
    };
}

ArgsInHash!()