// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "AstVisitor",
decl_type: "function",
source_file: "./src/ast_statistics.rs",
source_crate: ".",
deps: ["AstStatistics", "RdfStateMachine"],
uses: ["AstStatistics", "AstVisitor", "RdfStateMachine"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        AstStatistics!();
        RdfStateMachine!();
    };
}

macro_rules! AstVisitor {
    () => {
        deps!();
        struct AstVisitor < 'a > { ast_stats : & 'a mut AstStatistics , rdf_state : & 'a mut RdfStateMachine , }
    };
}

AstVisitor!();