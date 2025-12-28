macro_rules! PathError {
    () => {
        # [derive (Debug)] struct PathError { path : PathBuf , err : io :: Error , }
    };
}

PathError!();