macro_rules! inotify {
    () => {
        # [cfg (linux_raw_dep)] pub mod inotify ;
    };
}

inotify!();