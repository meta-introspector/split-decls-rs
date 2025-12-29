// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SynLangPatterns",
decl_type: "function",
source_file: "./src/meta_pattern_visitor.rs",
source_crate: ".",
deps: [],
uses: ["ItemUse", "ExprMatch", "Signature", "Copy", "ItemStatic", "ItemImpl", "PartialEq", "ExprBlock", "ItemFn", "TypeTuple", "Generics", "ExprCall", "Pat", "ExprBinary", "Debug", "Eq", "ItemConst", "PatIdent", "ItemTrait", "ExprUnary", "PatTuple", "Block", "Item", "Clone", "Ident", "Path", "ItemMod", "Expr", "SynLangPatterns", "File", "ExprIf", "Hash", "ItemEnum", "ExprMethodCall", "ExprPath", "Type", "TypePath", "PatStruct", "ExprLit", "ItemStruct", "TypeReference"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! SynLangPatterns {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum SynLangPatterns { File , Item , ItemFn , ItemStruct , ItemEnum , ItemImpl , ItemTrait , ItemMod , ItemUse , ItemConst , ItemStatic , Expr , ExprCall , ExprMethodCall , ExprPath , ExprLit , ExprBlock , ExprIf , ExprMatch , ExprBinary , ExprUnary , Type , TypePath , TypeReference , TypeTuple , Pat , PatIdent , PatStruct , PatTuple , Ident , Path , Block , Signature , Generics , }
    };
}

SynLangPatterns!();