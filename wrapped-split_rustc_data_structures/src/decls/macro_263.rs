macro_rules! macro_263 {
    () => {
        impls_dyn_send_neg ! ([std :: env :: Args] [std :: env :: ArgsOs] [* const T where T : ? Sized + PointeeSized] [* mut T where T : ? Sized + PointeeSized] [std :: ptr :: NonNull < T > where T : ? Sized + PointeeSized] [std :: rc :: Rc < T , A > where T : ? Sized , A : Allocator] [std :: rc :: Weak < T , A > where T : ? Sized , A : Allocator] [std :: sync :: MutexGuard <'_ , T > where T : ? Sized] [std :: sync :: RwLockReadGuard <'_ , T > where T : ? Sized] [std :: sync :: RwLockWriteGuard <'_ , T > where T : ? Sized] [std :: io :: StdoutLock <'_ >] [std :: io :: StderrLock <'_ >]) ;
    };
}

macro_263!()