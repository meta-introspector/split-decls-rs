// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "Term",
decl_type: "function",
source_file: "./src/macro_analyzer_parts/terms.rs",
source_crate: ".",
deps: [],
uses: ["BooleanLiteral", "Term", "Hash", "Debug", "Clone", "ByteLiteral", "PartialEq", "Serialize", "NumericLiteral", "Deserialize", "String", "FloatLiteral", "Eq", "FunctionCall", "StringLiteral", "Identifier", "CharLiteral"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! Term {
    () => {
        # [derive (Debug , Clone , PartialEq , Eq , Hash , Serialize , Deserialize)] pub enum Term { StringLiteral (String) , NumericLiteral (String) , BooleanLiteral (bool) , CharLiteral (char) , ByteLiteral (u8) , FloatLiteral (String) , Identifier (String) , FunctionCall (String) , }
    };
}

Term!();