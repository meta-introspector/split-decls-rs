// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "sparql_select",
decl_type: "function",
source_file: "./src/sparql_macros.rs",
source_crate: ".",
deps: [],
uses: ["SPARQL", "SELECT", "Convenience", "WHERE"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! sparql_select {
    () => {
        # [doc = " Convenience macro for common SPARQL patterns"] macro_rules ! sparql_select { ($ var : ident has_type $ type : ident) => { sparql ! (SELECT $ var WHERE { $ var rdf : type $ type }) } ; ($ var : ident has_property $ prop : ident as $ value : ident) => { sparql ! (SELECT $ var $ value WHERE { $ var $ prop $ value }) } ; }
    };
}

sparql_select!();