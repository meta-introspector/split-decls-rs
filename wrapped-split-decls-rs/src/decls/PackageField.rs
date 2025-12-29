// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "PackageField",
decl_type: "function",
source_file: "./src/cargo_guided_analysis.rs",
source_crate: ".",
deps: [],
uses: ["Option", "Deserialize", "Vec", "Serialize", "Clone", "String", "Debug", "PackageField"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! PackageField {
    () => {
        # [derive (Debug , Clone , Serialize , Deserialize)] pub struct PackageField { pub name : String , pub version : String , pub source : String , pub dependencies : Vec < String > , pub checksum : Option < String > , }
    };
}

PackageField!();