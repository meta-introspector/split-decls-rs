macro_rules! deps {
    () => {
        Success!();
        WithOverflow!();
        Cx!();
    };
}

macro_rules! CacheEntry {
    () => {
        deps!();
        # [doc = " The cache entry for a given input."] # [doc = ""] # [doc = " This contains results whose computation never hit the"] # [doc = " recursion limit in `success`, and all results which hit"] # [doc = " the recursion limit in `with_overflow`."] # [derive_where (Default ; X : Cx)] struct CacheEntry < X : Cx > { success : Option < Success < X > > , with_overflow : HashMap < usize , WithOverflow < X > > , }
    };
}

CacheEntry!();