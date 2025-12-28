macro_rules! LockState {
    () => {
        struct LockState { parallels : u32 , }
    };
}

LockState!();