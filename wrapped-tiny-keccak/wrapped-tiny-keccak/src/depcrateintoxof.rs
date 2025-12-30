// Generated macro for IntoXof (trait)
macro_rules! DepcrateIntoXof {
() => {
// Module: crate
// Provides: {"IntoXof"}
// Dependencies: {}
# [doc = " A trait used to convert [`Hasher`] into it's [`Xof`] counterpart."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use tiny_keccak::IntoXof;"] # [doc = " #"] # [doc = " # fn foo<H: IntoXof>(hasher: H) {"] # [doc = " let xof = hasher.into_xof();"] # [doc = " # }"] # [doc = " ```"] # [doc = ""] # [doc = " [`Hasher`]: trait.Hasher.html"] # [doc = " [`Xof`]: trait.Xof.html"] pub trait IntoXof { # [doc = " A type implementing [`Xof`], eXtendable-output function interface."] # [doc = ""] # [doc = " [`Xof`]: trait.Xof.html"] type Xof : Xof ; # [doc = " A method used to convert type into [`Xof`]."] # [doc = ""] # [doc = " [`Xof`]: trait.Xof.html"] fn into_xof (self) -> Self :: Xof ; }
};
}
