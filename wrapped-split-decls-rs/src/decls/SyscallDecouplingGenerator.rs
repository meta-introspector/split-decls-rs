// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SyscallDecouplingGenerator",
decl_type: "function",
source_file: "./src/syscall_decoupling_template.rs",
source_crate: ".",
deps: ["SyscallAnalysisReport"],
uses: ["HashMap", "SyscallDecouplingGenerator", "String", "SyscallAnalysisReport"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
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