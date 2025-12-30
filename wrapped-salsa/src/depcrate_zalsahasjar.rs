// Generated macro for HasJar (trait)
macro_rules! Depcrate_zalsaHasJar {
() => {
// Module: crate::zalsa
// Provides: {"HasJar"}
// Dependencies: {}
# [doc = " A salsa ingredient that can be registered in the database."] # [doc = ""] # [doc = " This trait is implemented for tracked functions and salsa structs."] pub trait HasJar { # [doc = " The [`Jar`] associated with this ingredient."] type Jar : Jar ; # [doc = " The [`JarKind`] for `Self::Jar`."] const KIND : JarKind ; }
};
}
