// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SynLangPatterns",
decl_type: "function",
source_file: "./src/meta_pattern_visitor.rs",
source_crate: ".",
deps: [],
uses: ["ItemStruct", "PartialEq", "ItemUse", "Hash", "TypePath", "ExprIf", "Signature", "ExprBinary", "ItemStatic", "Type", "Generics", "Expr", "Clone", "Ident", "ExprMethodCall", "Pat", "Path", "ItemConst", "ItemMod", "Item", "Copy", "ExprUnary", "File", "Block", "ItemFn", "ExprCall", "PatTuple", "Debug", "TypeReference", "PatStruct", "ItemEnum", "ExprMatch", "SynLangPatterns", "ExprBlock", "Eq", "ItemTrait", "ExprLit", "ExprPath", "TypeTuple", "ItemImpl", "PatIdent"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! SynLangPatterns {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum SynLangPatterns { File , Item , ItemFn , ItemStruct , ItemEnum , ItemImpl , ItemTrait , ItemMod , ItemUse , ItemConst , ItemStatic , Expr , ExprCall , ExprMethodCall , ExprPath , ExprLit , ExprBlock , ExprIf , ExprMatch , ExprBinary , ExprUnary , Type , TypePath , TypeReference , TypeTuple , Pat , PatIdent , PatStruct , PatTuple , Ident , Path , Block , Signature , Generics , }
    };
}

SynLangPatterns!();