macro_rules! deps {
    () => {
        FileLock!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl Drop for FileLock { fn drop (& mut self) { match self { FileLock :: NotLocked => { } FileLock :: Locked { path , done } => { done . store (true , Ordering :: Release) ; let _ = fs :: remove_file (path) ; } } } }
    };
}

impl_100!()