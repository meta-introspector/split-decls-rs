// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "println_sparql",
decl_type: "function",
source_file: "./src/sparql_macros.rs",
source_crate: ".",
deps: [],
uses: ["Print", "SPARQL"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! println_sparql {
    () => {
        # [doc = " Print a SPARQL query with proper formatting"] macro_rules ! println_sparql { ($ query : expr) => { println ! ("{}" , $ query) ; } ; ($ label : expr , $ query : expr) => { println ! ("\n{}. {}" , $ label , $ query) ; } ; }
    };
}

println_sparql!();