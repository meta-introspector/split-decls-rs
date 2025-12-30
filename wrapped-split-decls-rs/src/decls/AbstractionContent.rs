// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "AbstractionContent",
decl_type: "function",
source_file: "./src/bott_periodicity.rs",
source_crate: ".",
deps: [],
uses: ["Pattern", "MetaPattern", "Debug", "Meta-pattern", "Quaternionic", "Concrete", "Meta-meta-pattern", "AbstractionContent", "Level", "TokenStream", "Clone", "Dual", "Box", "MetaMetaPattern", "String"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! AbstractionContent {
    () => {
        # [derive (Debug , Clone)] pub enum AbstractionContent { # [doc = " Level 0, 8, 16...: Concrete code (0-dimensional)"] Concrete (TokenStream) , # [doc = " Level 1, 9, 17...: Pattern (1-dimensional bundle)"] Pattern { template : String , fiber_dim : usize , } , # [doc = " Level 2, 10, 18...: Meta-pattern (back to 0-dimensional!)"] MetaPattern { meta_structure : String , } , # [doc = " Level 3, 11, 19...: Meta-meta-pattern (1-dimensional again)"] MetaMetaPattern { structure : String , fiber_dim : usize , } , # [doc = " Level 4, 12, 20...: Quaternionic (4-dimensional symmetry)"] Quaternionic { real_part : String , imag_parts : [String ; 3] , } , # [doc = " Level 5-7: Dual structures (descent)"] Dual { level : usize , base : Box < AbstractionContent > , } , }
    };
}

AbstractionContent!();