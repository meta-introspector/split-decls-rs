macro_rules! any {
    () => {
        # [cfg (not (unix))] mod any ;
    };
}

any!();