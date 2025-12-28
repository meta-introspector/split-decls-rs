macro_rules! Align32 {
    () => {
        # [repr (align (32))] struct Align32 < T > (T) ;
    };
}

Align32!();