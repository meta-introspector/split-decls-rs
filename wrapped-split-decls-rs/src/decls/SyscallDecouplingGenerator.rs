// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SyscallDecouplingGenerator",
decl_type: "function",
source_file: "./src/syscall_decoupling_template.rs",
source_crate: ".",
deps: ["SyscallAnalysisReport"],
uses: ["String", "SyscallDecouplingGenerator", "HashMap", "SyscallAnalysisReport"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        SyscallAnalysisReport!();
    };
}

macro_rules! SyscallDecouplingGenerator {
    () => {
        deps!();
        pub struct SyscallDecouplingGenerator { analysis_report : SyscallAnalysisReport , sparql_complexity_data : HashMap < String , f64 > , }
    };
}

SyscallDecouplingGenerator!();