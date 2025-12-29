// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "create_example_probes",
decl_type: "function",
source_file: "./src/ast_reflector.rs",
source_crate: ".",
deps: ["ProbeFilter", "ProbeAction", "AstNodeType", "AstProbe"],
uses: ["ProbeFilter", "AddAttribute", "Function", "Collect", "ProbeAction", "Struct", "Enhance", "High", "AstNodeType", "WrapFunction", "Vec", "AstProbe", "None", "Some"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        ProbeFilter!();
        ProbeAction!();
        AstNodeType!();
        AstProbe!();
    };
}

macro_rules! create_example_probes {
    () => {
        deps!();
        pub fn create_example_probes () -> Vec < AstProbe > { vec ! [AstProbe { name : "debug_functions" . to_string () , node_type : AstNodeType :: Function , filter : ProbeFilter { name_pattern : Some ("debug" . to_string ()) , visibility : None , attributes : vec ! [] , contains_text : None , complexity_threshold : None , layer : None , } , action : ProbeAction :: AddAttribute { attr : "#[instrument]" . to_string () , } , enabled : true , priority : 1 , } , AstProbe { name : "collect_structs" . to_string () , node_type : AstNodeType :: Struct , filter : ProbeFilter { name_pattern : None , visibility : Some ("pub" . to_string ()) , attributes : vec ! [] , contains_text : None , complexity_threshold : None , layer : None , } , action : ProbeAction :: Collect { field : "public_structs" . to_string () , } , enabled : true , priority : 2 , } , AstProbe { name : "enhance_complex_functions" . to_string () , node_type : AstNodeType :: Function , filter : ProbeFilter { name_pattern : None , visibility : None , attributes : vec ! [] , contains_text : None , complexity_threshold : Some (5.0) , layer : None , } , action : ProbeAction :: Enhance { enhancement_type : "complexity_warning" . to_string () , data : "High complexity function detected" . to_string () , } , enabled : true , priority : 3 , } , AstProbe { name : "wrap_error_functions" . to_string () , node_type : AstNodeType :: Function , filter : ProbeFilter { name_pattern : Some ("error" . to_string ()) , visibility : None , attributes : vec ! [] , contains_text : None , complexity_threshold : None , layer : None , } , action : ProbeAction :: WrapFunction { wrapper : "error_handler" . to_string () , } , enabled : true , priority : 4 , } ,] }
    };
}

create_example_probes!();