macro_rules! tty {
    () => {
        # [cfg (not (windows))] mod tty ;
    };
}

tty!()