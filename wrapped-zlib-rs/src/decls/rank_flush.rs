macro_rules! rank_flush {
    () => {
        const fn rank_flush (f : i8) -> i8 { ((f) * 2) - (if (f) > 4 { 9 } else { 0 }) }
    };
}

rank_flush!()