// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "ImplAdapter",
decl_type: "function",
source_file: "./src/syscall_decoupling_template.rs",
source_crate: ".",
deps: ["ImplementationType"],
uses: ["ImplAdapter", "Serialize", "String", "ImplementationType", "Debug", "Deserialize"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        ImplementationType!();
    };
}

macro_rules! ImplAdapter {
    () => {
        deps!();
        # [derive (Debug , Serialize , Deserialize)] pub struct ImplAdapter { pub trait_name : String , pub implementation_type : ImplementationType , pub mock_variant : bool , }
    };
}

ImplAdapter!();