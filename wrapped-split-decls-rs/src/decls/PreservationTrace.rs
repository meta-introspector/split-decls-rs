// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "PreservationTrace",
decl_type: "function",
source_file: "./src/cargo_guided_analysis.rs",
source_crate: ".",
deps: ["PackageField"],
uses: ["PreservationTrace", "Deserialize", "Clone", "PackageField", "Vec", "Option", "Debug", "Serialize", "String"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
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