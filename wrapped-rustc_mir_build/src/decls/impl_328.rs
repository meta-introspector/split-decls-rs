macro_rules! deps {
    () => {
        PatCtxt!();
        ConstToPat!();
    };
}

macro_rules! impl_328 {
    () => {
        deps!();
        impl < 'a , 'tcx > PatCtxt < 'a , 'tcx > { # [doc = " Converts a constant to a pattern (if possible)."] # [doc = " This means aggregate values (like structs and enums) are converted"] # [doc = " to a pattern that matches the value (as if you'd compared via structural equality)."] # [doc = ""] # [doc = " Only type system constants are supported, as we are using valtrees"] # [doc = " as an intermediate step. Unfortunately those don't carry a type"] # [doc = " so we have to carry one ourselves."] # [instrument (level = "debug" , skip (self) , ret)] pub (super) fn const_to_pat (& self , c : ty :: Const < 'tcx > , ty : Ty < 'tcx > , id : hir :: HirId , span : Span ,) -> Box < Pat < 'tcx > > { let mut convert = ConstToPat :: new (self , id , span , c) ; match c . kind () { ty :: ConstKind :: Unevaluated (uv) => convert . unevaluated_to_pat (uv , ty) , ty :: ConstKind :: Value (cv) => convert . valtree_to_pat (cv . valtree , cv . ty) , _ => span_bug ! (span , "Invalid `ConstKind` for `const_to_pat`: {:?}" , c) , } } }
    };
}

impl_328!()