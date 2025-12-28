macro_rules! deps {
    () => {
        Level!();
        LevelFilter!();
    };
}

macro_rules! filter_as_usize {
    () => {
        deps!();
        # [inline (always)] fn filter_as_usize (x : & Option < Level >) -> usize { match x { Some (Level (f)) => * f as usize , None => LevelFilter :: OFF_USIZE , } }
    };
}

filter_as_usize!();