macro_rules! digits {
    () => {
        fn digits (val : u64) -> usize { if val < 10 { 1 } else { 1 + digits (val / 10) } }
    };
}

digits!();