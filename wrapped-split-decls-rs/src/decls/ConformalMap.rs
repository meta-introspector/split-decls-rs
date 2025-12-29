// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "ConformalMap",
decl_type: "function",
source_file: "./src/conformal_field_theory.rs",
source_crate: ".",
deps: ["Level8DPoint", "AnglePreservation"],
uses: ["Debug", "Clone", "Deserialize", "ConformalMap", "Vec", "Serialize", "Level8DPoint", "HashMap", "String", "AnglePreservation"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        Level8DPoint!();
        AnglePreservation!();
    };
}

macro_rules! ConformalMap {
    () => {
        deps!();
        # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ConformalMap { pub phi_8d : Level8DPoint , pub c1_to_n1 : HashMap < String , String > , pub n1_to_c2 : HashMap < String , String > , pub angle_preservation : Vec < AnglePreservation > , }
    };
}

ConformalMap!();