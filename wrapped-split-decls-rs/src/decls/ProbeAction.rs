// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "ProbeAction",
decl_type: "function",
source_file: "./src/ast_reflector.rs",
source_crate: ".",
deps: ["InjectionPosition"],
uses: ["Deserialize", "Clone", "Collect", "String", "InjectCode", "WrapFunction", "InjectionPosition", "Serialize", "Transform", "Log", "ProbeAction", "Vec", "AddAttribute", "Enhance", "Debug"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        InjectionPosition!();
    };
}

macro_rules! ProbeAction {
    () => {
        deps!();
        # [derive (Debug , Clone , Serialize , Deserialize)] pub enum ProbeAction { Log { message : String } , AddAttribute { attr : String } , WrapFunction { wrapper : String } , InjectCode { code : String , position : InjectionPosition } , Transform { macro_name : String , args : Vec < String > } , Collect { field : String } , Enhance { enhancement_type : String , data : String } , }
    };
}

ProbeAction!();