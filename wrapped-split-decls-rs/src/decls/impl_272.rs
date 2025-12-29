// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "impl_272",
decl_type: "function",
source_file: "./src/syn_type_discovery.rs",
source_crate: ".",
deps: ["TypeExtractorVisitor"],
uses: ["Pat", "Expr", "Item", "Stmt", "TypeExtractorVisitor", "Lit", "Type"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        TypeExtractorVisitor!();
    };
}

macro_rules! impl_272 {
    () => {
        deps!();
        impl TypeExtractorVisitor < '_ > { fn is_syn_ast_enum (& self , name : & str) -> bool { matches ! (name , "Item" | "Expr" | "Type" | "Pat" | "Stmt" | "Lit") || name . starts_with ("Item") || name . starts_with ("Expr") || name . starts_with ("Type") || name . starts_with ("Pat") || name . starts_with ("Lit") } }
    };
}

impl_272!();