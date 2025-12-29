// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "sparql",
decl_type: "function",
source_file: "./src/sparql_macros.rs",
source_crate: ".",
deps: [],
uses: ["SPARQL", "Variables", "SELECT", "WHERE", "This", "CONSTRUCT", "ASK", "Main"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! sparql {
    () => {
        # [doc = " SPARQL macro system for safe query construction"] # [doc = " "] # [doc = " This macro system allows constructing SPARQL queries without format string issues."] # [doc = " Variables are prefixed with $ and the macro handles proper escaping."] # [doc = " Main SPARQL query construction macro"] macro_rules ! sparql { (SELECT $ ($ var : ident) + WHERE { $ ($ triple : tt) * }) => { format ! ("SELECT {} WHERE {{ {} }}" , sparql ! (@ vars $ ($ var) +) , sparql ! (@ triples $ ($ triple) *)) } ; (ASK WHERE { $ ($ triple : tt) * }) => { format ! ("ASK WHERE {{ {} }}" , sparql ! (@ triples $ ($ triple) *)) } ; (CONSTRUCT { $ ($ construct : tt) * } WHERE { $ ($ where : tt) * }) => { format ! ("CONSTRUCT {{ {} }} WHERE {{ {} }}" , sparql ! (@ triples $ ($ construct) *) , sparql ! (@ triples $ ($ where) *)) } ; (@ vars $ var : ident) => { format ! ("?{}" , stringify ! ($ var)) } ; (@ vars $ var : ident $ ($ rest : ident) +) => { format ! ("?{} {}" , stringify ! ($ var) , sparql ! (@ vars $ ($ rest) +)) } ; (@ triples $ s : ident $ p : ident $ o : ident) => { format ! ("?{} {}:{} ?{}" , stringify ! ($ s) , "lmdfb" , stringify ! ($ p) , stringify ! ($ o)) } ; (@ triples $ s : ident $ p : ident $ o : literal) => { format ! ("?{} {}:{} {}" , stringify ! ($ s) , "lmdfb" , stringify ! ($ p) , $ o) } ; (@ triples $ s : ident rdf : type $ o : ident) => { format ! ("?{} rdf:type lmdfb:{}" , stringify ! ($ s) , stringify ! ($ o)) } ; (@ triples $ s : ident $ p : ident $ o : ident . $ ($ rest : tt) *) => { format ! ("?{} {}:{} ?{} . {}" , stringify ! ($ s) , "lmdfb" , stringify ! ($ p) , stringify ! ($ o) , sparql ! (@ triples $ ($ rest) *)) } ; (@ triples) => { "" } ; }
    };
}

sparql!();