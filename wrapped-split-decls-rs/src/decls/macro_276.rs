// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "macro_276",
decl_type: "function",
source_file: "./src/meta_pattern_visitor.rs",
source_crate: ".",
deps: ["SynLangPatterns"],
uses: ["ExprBinary", "ItemConst", "TypePath", "Item", "File", "Type", "ItemEnum", "Signature", "ExprMatch", "Pat", "Expr", "Generics", "ItemStatic", "PatIdent", "ExprMethodCall", "ExprCall", "ExprLit", "TypeTuple", "ExprBlock", "ExprUnary", "ItemStruct", "Block", "Path", "ItemMod", "ItemFn", "ItemUse", "SynLangPatterns", "ExprPath", "ExprIf", "PatStruct", "PatTuple", "TypeReference", "ItemTrait", "Ident", "ItemImpl"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
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