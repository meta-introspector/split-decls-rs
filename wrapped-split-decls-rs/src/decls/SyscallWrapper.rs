// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SyscallWrapper",
decl_type: "function",
source_file: "./src/syscall_oracle.rs",
source_crate: ".",
deps: ["OracleType"],
uses: ["Debug", "Serialize", "Option", "String", "SyscallWrapper", "OracleType", "Deserialize", "Clone"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        OracleType!();
    };
}

macro_rules! SyscallWrapper {
    () => {
        deps!();
        # [derive (Debug , Clone , Serialize , Deserialize)] pub struct SyscallWrapper { pub original_call : String , pub wrapper_macro : String , pub oracle_type : OracleType , pub safety_wrapper : String , pub mock_implementation : Option < String > , pub dao_policy : Option < String > , }
    };
}

SyscallWrapper!();