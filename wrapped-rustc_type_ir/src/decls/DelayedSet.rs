macro_rules! DelayedSet {
    () => {
        # [derive (Debug)] pub struct DelayedSet < T > { cache : HashSet < T > , count : u32 , }
    };
}

DelayedSet!();