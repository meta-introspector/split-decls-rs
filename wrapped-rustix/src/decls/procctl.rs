macro_rules! procctl {
    () => {
        # [cfg (freebsdlike)] mod procctl ;
    };
}

procctl!();