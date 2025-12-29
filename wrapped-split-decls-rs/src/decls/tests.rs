// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "tests",
decl_type: "function",
source_file: "./src/sparql_macros.rs",
source_crate: ".",
deps: [],
uses: ["RustcComponent", "WHERE", "SELECT"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! tests {
    () => {
        # [cfg (test)] mod tests { # [test] fn test_sparql_macros () { let query1 = sparql ! (SELECT s WHERE { s rdf : type RustcComponent }) ; assert_eq ! (query1 , "SELECT ?s WHERE { ?s rdf:type lmdfb:RustcComponent }") ; let query2 = sparql_select ! (s has_type RustcComponent) ; assert_eq ! (query2 , "SELECT ?s WHERE { ?s rdf:type lmdfb:RustcComponent }") ; let query3 = sparql_select ! (s has_property hasLevel as level) ; assert_eq ! (query3 , "SELECT ?s ?level WHERE { ?s lmdfb:hasLevel ?level }") ; } }
    };
}

tests!();