// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "cached_read_to_string",
decl_type: "function",
source_file: "./src/syscall.rs",
source_crate: ".",
deps: [],
uses: ["Ok", "String", "AsRef", "Path", "Result", "GLOBAL_CACHE"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! cached_read_to_string {
    () => {
        pub fn cached_read_to_string < P : AsRef < Path > > (path : P) -> anyhow :: Result < String > { let mut cache = GLOBAL_CACHE . lock () . unwrap () ; let content = cache . get_cached_content (path . as_ref ()) ? ; let _ = cache . save () ; Ok (content) }
    };
}

cached_read_to_string!();