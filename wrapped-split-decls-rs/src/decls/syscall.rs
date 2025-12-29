// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "syscall",
decl_type: "function",
source_file: "./src/lib.rs",
source_crate: ".",
deps: [],
uses: [],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! syscall {
    () => {
        pub mod syscall ;
    };
}

syscall!();