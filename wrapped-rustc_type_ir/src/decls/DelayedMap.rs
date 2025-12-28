macro_rules! DelayedMap {
    () => {
        # [doc = " A hashmap which only starts hashing after ignoring the first few inputs."] # [doc = ""] # [doc = " This is used in type folders as in nearly all cases caching is not worth it"] # [doc = " as nearly all folded types are tiny. However, there are very rare incredibly"] # [doc = " large types for which caching is necessary to avoid hangs."] # [derive (Debug)] pub struct DelayedMap < K , V > { cache : HashMap < K , V > , count : u32 , }
    };
}

DelayedMap!();