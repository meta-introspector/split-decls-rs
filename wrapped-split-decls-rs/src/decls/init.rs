// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "init",
decl_type: "function",
source_file: "./src/syscall.rs",
source_crate: ".",
deps: ["SyscallTracker"],
uses: ["SyscallTracker"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        SyscallTracker!();
    };
}

macro_rules! init {
    () => {
        deps!();
        pub fn init () -> SyscallTracker { SyscallTracker :: new () }
    };
}

init!();