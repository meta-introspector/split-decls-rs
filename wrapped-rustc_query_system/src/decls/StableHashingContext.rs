macro_rules! StableHashingContext {
    () => {
        # [doc = " This is the context state available during incr. comp. hashing. It contains"] # [doc = " enough information to transform `DefId`s and `HirId`s into stable `DefPath`s (i.e.,"] # [doc = " a reference to the `TyCtxt`) and it holds a few caches for speeding up various"] # [doc = " things (e.g., each `DefId`/`DefPath` is only hashed once)."] # [derive (Clone)] pub struct StableHashingContext < 'a > { untracked : & 'a Untracked , incremental_ignore_spans : bool , raw_source_map : & 'a SourceMap , caching_source_map : Option < CachingSourceMapView < 'a > > , hashing_controls : HashingControls , }
    };
}

StableHashingContext!();