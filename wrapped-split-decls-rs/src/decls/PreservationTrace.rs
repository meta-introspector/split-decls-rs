// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "PreservationTrace",
decl_type: "function",
source_file: "./src/cargo_guided_analysis.rs",
source_crate: ".",
deps: ["PackageField"],
uses: ["Deserialize", "String", "Debug", "Option", "Serialize", "PreservationTrace", "Clone", "PackageField", "Vec"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        PackageField!();
    };
}

macro_rules! PreservationTrace {
    () => {
        deps!();
        # [derive (Debug , Clone , Serialize , Deserialize)] pub struct PreservationTrace { pub original_package : PackageField , pub toml_reference : Option < String > , pub bootstrap_reference : Option < String > , pub output2_reference : Option < String > , pub preserved : bool , pub transformation_steps : Vec < String > , }
    };
}

PreservationTrace!();