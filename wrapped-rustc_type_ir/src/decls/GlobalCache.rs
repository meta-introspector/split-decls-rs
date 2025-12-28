macro_rules! deps {
    () => {
        Cx!();
        CacheEntry!();
    };
}

macro_rules! GlobalCache {
    () => {
        deps!();
        # [derive_where (Default ; X : Cx)] pub struct GlobalCache < X : Cx > { map : HashMap < X :: Input , CacheEntry < X > > , }
    };
}

GlobalCache!();