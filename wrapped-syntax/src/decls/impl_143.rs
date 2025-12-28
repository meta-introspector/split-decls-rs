macro_rules! deps {
    () => {
        SyntaxNode!();
        SyntaxAnnotation!();
        SyntaxElement!();
        SyntaxEdit!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl SyntaxEdit { # [doc = " Root of the initial unmodified syntax tree."] pub fn old_root (& self) -> & SyntaxNode { & self . old_root } # [doc = " Root of the modified syntax tree."] pub fn new_root (& self) -> & SyntaxNode { & self . new_root } # [doc = " Which syntax elements in the modified syntax tree were inserted or"] # [doc = " modified as part of the edit."] # [doc = ""] # [doc = " Note that for syntax nodes, only the upper-most parent of a set of"] # [doc = " changes is included, not any child elements that may have been modified."] pub fn changed_elements (& self) -> & [SyntaxElement] { self . changed_elements . as_slice () } # [doc = " Finds which syntax elements have been annotated with the given"] # [doc = " annotation."] # [doc = ""] # [doc = " Note that an annotation might not appear in the modified syntax tree if"] # [doc = " the syntax elements that were annotated did not make it into the final"] # [doc = " syntax tree."] pub fn find_annotation (& self , annotation : SyntaxAnnotation) -> & [SyntaxElement] { self . annotations . get (& annotation) . as_ref () . map_or (& [] , | it | it . as_slice ()) } }
    };
}

impl_143!()