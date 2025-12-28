macro_rules! SyntaxContext {
    () => {
        # [cfg (not (feature = "salsa"))] # [derive (Copy , Clone , PartialEq , PartialOrd , Eq , Ord , Hash)] pub struct SyntaxContext (u32) ;
    };
}

SyntaxContext!();