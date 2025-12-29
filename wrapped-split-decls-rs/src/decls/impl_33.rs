// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "impl_33",
decl_type: "function",
source_file: "./src/macro_analyzer_parts/terms.rs",
source_crate: ".",
deps: ["Term"],
uses: ["Identifier", "Display", "BooleanLiteral", "StringLiteral", "CharLiteral", "Formatter", "ByteLiteral", "Result", "FloatLiteral", "Term", "NumericLiteral", "FunctionCall"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        Term!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl fmt :: Display for Term { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { Term :: StringLiteral (s) => write ! (f , "{}" , s) , Term :: NumericLiteral (s) => write ! (f , "{}" , s) , Term :: BooleanLiteral (b) => write ! (f , "{}" , b) , Term :: CharLiteral (c) => write ! (f , "{}" , c) , Term :: ByteLiteral (b) => write ! (f , "{}" , b) , Term :: FloatLiteral (s) => write ! (f , "{}" , s) , Term :: Identifier (s) => write ! (f , "{}" , s) , Term :: FunctionCall (s) => write ! (f , "{}" , s) , } } }
    };
}

impl_33!();