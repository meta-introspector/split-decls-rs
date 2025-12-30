// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "Term",
decl_type: "function",
source_file: "./src/macro_analyzer_parts/terms.rs",
source_crate: ".",
deps: [],
uses: ["BooleanLiteral", "Identifier", "Hash", "CharLiteral", "String", "Clone", "Eq", "FunctionCall", "Term", "Debug", "Serialize", "Deserialize", "StringLiteral", "ByteLiteral", "NumericLiteral", "PartialEq", "FloatLiteral"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! Term {
    () => {
        # [derive (Debug , Clone , PartialEq , Eq , Hash , Serialize , Deserialize)] pub enum Term { StringLiteral (String) , NumericLiteral (String) , BooleanLiteral (bool) , CharLiteral (char) , ByteLiteral (u8) , FloatLiteral (String) , Identifier (String) , FunctionCall (String) , }
    };
}

Term!();