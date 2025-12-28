macro_rules! BoolExt {
    () => {
        # [allow (unused)] trait BoolExt { fn then_some < T > (self , t : T) -> Option < T > ; }
    };
}

BoolExt!();