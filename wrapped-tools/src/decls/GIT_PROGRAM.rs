macro_rules! GIT_PROGRAM {
    () => {
        # [cfg (not (windows))] const GIT_PROGRAM : & str = "git" ;
    };
}

GIT_PROGRAM!()