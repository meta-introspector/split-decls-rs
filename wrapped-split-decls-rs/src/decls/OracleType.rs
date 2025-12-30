// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "OracleType",
decl_type: "function",
source_file: "./src/syscall_oracle.rs",
source_crate: ".",
deps: [],
uses: ["Memory", "Crypto", "Custom", "Clone", "OracleType", "Debug", "FileSystem", "Network", "Serialize", "Process", "String", "Deserialize", "Time"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! OracleType {
    () => {
        # [derive (Debug , Clone , Serialize , Deserialize)] pub enum OracleType { FileSystem , Network , Process , Memory , Time , Crypto , Custom (String) , }
    };
}

OracleType!();