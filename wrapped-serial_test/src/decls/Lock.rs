macro_rules! Lock {
    () => {
        pub (crate) struct Lock { lockfile : LockFile , pub (crate) parallel_count : u32 , path : String , }
    };
}

Lock!()