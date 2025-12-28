macro_rules! SyntaxAnnotation {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] # [repr (transparent)] pub struct SyntaxAnnotation (NonZeroU32) ;
    };
}

SyntaxAnnotation!();