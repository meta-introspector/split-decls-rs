// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "macro_276",
decl_type: "function",
source_file: "./src/meta_pattern_visitor.rs",
source_crate: ".",
deps: ["SynLangPatterns"],
uses: ["Expr", "Type", "ExprUnary", "Pat", "PatIdent", "PatStruct", "ExprPath", "Signature", "ItemMod", "TypePath", "TypeTuple", "Item", "ItemUse", "ExprBlock", "ExprMethodCall", "Generics", "ItemEnum", "ItemStatic", "ExprBinary", "ItemTrait", "ItemConst", "ExprCall", "Path", "Ident", "ItemStruct", "Block", "ItemFn", "TypeReference", "SynLangPatterns", "PatTuple", "ExprLit", "ExprMatch", "ItemImpl", "File", "ExprIf"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        SynLangPatterns!();
    };
}

macro_rules! macro_276 {
    () => {
        deps!();
        mkmeta ! (SynLangPatterns { File , Item , ItemFn , ItemStruct , ItemEnum , ItemImpl , ItemTrait , ItemMod , ItemUse , ItemConst , ItemStatic , Expr , ExprCall , ExprMethodCall , ExprPath , ExprLit , ExprBlock , ExprIf , ExprMatch , ExprBinary , ExprUnary , Type , TypePath , TypeReference , TypeTuple , Pat , PatIdent , PatStruct , PatTuple , Ident , Path , Block , Signature , Generics , }) ;
    };
}

macro_276!();