// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "macro_246",
decl_type: "function",
source_file: "./src/syscall.rs",
source_crate: ".",
deps: ["SynCache"],
uses: ["Mutex", "GLOBAL_CACHE", "SynCache"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        SynCache!();
    };
}

macro_rules! macro_246 {
    () => {
        deps!();
        lazy_static :: lazy_static ! { static ref GLOBAL_CACHE : Mutex < SynCache > = Mutex :: new (SynCache :: new (".syn_cache.json")) ; }
    };
}

macro_246!();