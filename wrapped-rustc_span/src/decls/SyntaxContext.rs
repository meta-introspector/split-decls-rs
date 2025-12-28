macro_rules! deps {
    () => {
        Transparency!();
        ExpnId!();
    };
}

macro_rules! SyntaxContext {
    () => {
        deps!();
        # [doc = " A `SyntaxContext` represents a chain of pairs `(ExpnId, Transparency)` named \"marks\"."] # [doc = ""] # [doc = " See <https://rustc-dev-guide.rust-lang.org/macro-expansion.html> for more explanation."] # [derive (Clone , Copy , PartialEq , Eq , Hash)] pub struct SyntaxContext (u32) ;
    };
}

SyntaxContext!();