// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "OracleType",
decl_type: "function",
source_file: "./src/syscall_oracle.rs",
source_crate: ".",
deps: [],
uses: ["Serialize", "Debug", "Time", "OracleType", "Clone", "Deserialize", "Network", "Crypto", "FileSystem", "Process", "Custom", "Memory", "String"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! OracleType {
    () => {
        # [derive (Debug , Clone , Serialize , Deserialize)] pub enum OracleType { FileSystem , Network , Process , Memory , Time , Crypto , Custom (String) , }
    };
}

OracleType!();