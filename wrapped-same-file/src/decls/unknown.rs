macro_rules! unknown {
    () => {
        # [cfg (not (any (unix , windows)))] mod unknown ;
    };
}

unknown!();