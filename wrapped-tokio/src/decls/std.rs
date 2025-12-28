macro_rules! std {
    () => {
        # [cfg (not (all (test , loom)))] mod std ;
    };
}

std!();