// Generated macro for must_be_unique (function)
macro_rules! Depcrate_arcmust_be_unique {
() => {
// Module: crate::arc
// Provides: {"must_be_unique"}
// Dependencies: {}
# [track_caller] fn must_be_unique < T : ? Sized > (arc : & mut Arc < T >) -> & mut UniqueArc < T > { match Arc :: try_as_unique (arc) { Ok (unique) => unique , Err (this) => panic ! ("`Arc` must be unique in order for this operation to be safe, there are currently {} copies" , Arc :: count (this)) , } }
};
}
