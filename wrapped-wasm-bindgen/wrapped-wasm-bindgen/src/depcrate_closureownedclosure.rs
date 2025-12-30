// Generated macro for OwnedClosure (struct)
macro_rules! Depcrate_closureOwnedClosure {
() => {
// Module: crate::closure
// Provides: {"OwnedClosure"}
// Dependencies: {}
# [doc = " Internal representation of the actual owned closure which we send to the JS"] # [doc = " in the constructor to convert it into a JavaScript value."] # [repr (transparent)] struct OwnedClosure < T : ? Sized > (Box < T >) ;
};
}
