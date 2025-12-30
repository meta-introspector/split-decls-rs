// Generated macro for ArcUnionBorrow (enum)
macro_rules! Depcrate_arc_unionArcUnionBorrow {
() => {
// Module: crate::arc_union
// Provides: {"ArcUnionBorrow"}
// Dependencies: {}
# [doc = " This represents a borrow of an `ArcUnion`."] # [derive (Debug)] pub enum ArcUnionBorrow < 'a , A : 'a , B : 'a > { First (ArcBorrow < 'a , A >) , Second (ArcBorrow < 'a , B >) , }
};
}
