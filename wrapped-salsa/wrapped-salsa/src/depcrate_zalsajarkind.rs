// Generated macro for JarKind (enum)
macro_rules! Depcrate_zalsaJarKind {
() => {
// Module: crate::zalsa
// Provides: {"JarKind"}
// Dependencies: {}
# [doc = " The kind of an `Jar`."] # [doc = ""] # [doc = " Note that the ordering of the variants is important. Struct ingredients must be"] # [doc = " initialized before tracked functions, as tracked function ingredients depend on"] # [doc = " their input struct."] # [derive (PartialEq , Eq , PartialOrd , Ord , Clone , Copy , Debug)] pub enum JarKind { # [doc = " An input/tracked/interned struct."] Struct , # [doc = " A tracked function."] TrackedFn , }
};
}
