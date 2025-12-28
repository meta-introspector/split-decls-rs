macro_rules! deps {
    () => {
        Arg!();
        Result!();
    };
}

macro_rules! add_watch {
    () => {
        deps!();
        # [doc = " `inotify_add_watch(self, path, flags)`—Adds a watch to inotify."] # [doc = ""] # [doc = " This registers or updates a watch for the filesystem path `path` and"] # [doc = " returns a watch descriptor corresponding to this watch."] # [doc = ""] # [doc = " Note: Due to the existence of hardlinks, providing two different paths to"] # [doc = " this method may result in it returning the same watch descriptor. An"] # [doc = " application should keep track of this externally to avoid logic errors."] # [doc (alias = "inotify_add_watch")] # [inline] pub fn add_watch < P : crate :: path :: Arg , Fd : AsFd > (inot : Fd , path : P , flags : inotify :: WatchFlags ,) -> io :: Result < i32 > { path . into_with_c_str (| path | syscalls :: inotify_add_watch (inot . as_fd () , path , flags)) }
    };
}

add_watch!()