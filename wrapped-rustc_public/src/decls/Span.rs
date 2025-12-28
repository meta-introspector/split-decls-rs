macro_rules! Span {
    () => {
        # [derive (Clone , Copy , PartialEq , Eq , Hash , Serialize)] pub struct Span (usize) ;
    };
}

Span!();