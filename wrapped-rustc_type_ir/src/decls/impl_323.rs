macro_rules! deps {
    () => {
        Interner!();
        TypingMode!();
    };
}

macro_rules! impl_323 {
    () => {
        deps!();
        impl < I : Interner > TypingMode < I > { # [doc = " Analysis outside of a body does not define any opaque types."] pub fn non_body_analysis () -> TypingMode < I > { TypingMode :: Analysis { defining_opaque_types_and_generators : Default :: default () } } pub fn typeck_for_body (cx : I , body_def_id : I :: LocalDefId) -> TypingMode < I > { TypingMode :: Analysis { defining_opaque_types_and_generators : cx . opaque_types_and_coroutines_defined_by (body_def_id) , } } # [doc = " While typechecking a body, we need to be able to define the opaque"] # [doc = " types defined by that body."] # [doc = ""] # [doc = " FIXME: This will be removed because it's generally not correct to define"] # [doc = " opaques outside of HIR typeck."] pub fn analysis_in_body (cx : I , body_def_id : I :: LocalDefId) -> TypingMode < I > { TypingMode :: Analysis { defining_opaque_types_and_generators : cx . opaque_types_defined_by (body_def_id) , } } pub fn borrowck (cx : I , body_def_id : I :: LocalDefId) -> TypingMode < I > { let defining_opaque_types = cx . opaque_types_defined_by (body_def_id) ; if defining_opaque_types . is_empty () { TypingMode :: non_body_analysis () } else { TypingMode :: Borrowck { defining_opaque_types } } } pub fn post_borrowck_analysis (cx : I , body_def_id : I :: LocalDefId) -> TypingMode < I > { let defined_opaque_types = cx . opaque_types_defined_by (body_def_id) ; if defined_opaque_types . is_empty () { TypingMode :: non_body_analysis () } else { TypingMode :: PostBorrowckAnalysis { defined_opaque_types } } } }
    };
}

impl_323!();