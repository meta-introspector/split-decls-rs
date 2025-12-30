// Generated macro for op (function)
macro_rules! Depcrate_parseop {
() => {
// Module: crate::parse
// Provides: {"op"}
// Dependencies: {}
fn op (input : & str) -> (Op , & str) { let bytes = input . as_bytes () ; if bytes . first () == Some (& b'=') { (Op :: Exact , & input [1 ..]) } else if bytes . first () == Some (& b'>') { if bytes . get (1) == Some (& b'=') { (Op :: GreaterEq , & input [2 ..]) } else { (Op :: Greater , & input [1 ..]) } } else if bytes . first () == Some (& b'<') { if bytes . get (1) == Some (& b'=') { (Op :: LessEq , & input [2 ..]) } else { (Op :: Less , & input [1 ..]) } } else if bytes . first () == Some (& b'~') { (Op :: Tilde , & input [1 ..]) } else if bytes . first () == Some (& b'^') { (Op :: Caret , & input [1 ..]) } else { (Op :: DEFAULT , input) } }
};
}
