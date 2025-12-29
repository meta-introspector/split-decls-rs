// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "create_pattern_visitor",
decl_type: "function",
source_file: "./src/meta_pattern_visitor.rs",
source_crate: ".",
deps: ["RdfStateMachine", "AstStatistics"],
uses: ["RdfStateMachine", "AstStatistics"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        RdfStateMachine!();
        AstStatistics!();
    };
}

macro_rules! create_pattern_visitor {
    () => {
        deps!();
        macro_rules ! create_pattern_visitor { ($ visitor_name : ident , $ patterns : expr) => { struct $ visitor_name <'a > { stats : &'a mut crate :: ast_statistics :: AstStatistics , rdf_state : &'a mut crate :: macro_interpreter :: RdfStateMachine , } impl $ visitor_name <'_ > { fn increment (& mut self , pattern_name : & str) { self . stats . increment (pattern_name) ; self . rdf_state . capture_data ("ast_pattern" , pattern_name) ; } } create_visitor_methods ! ($ visitor_name , $ patterns) ; } ; }
    };
}

create_pattern_visitor!();