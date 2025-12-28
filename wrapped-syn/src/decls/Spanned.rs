macro_rules! Spanned {
    () => {
        # [doc = " A trait that can provide the `Span` of the complete contents of a syntax"] # [doc = " tree node."] # [doc = ""] # [doc = " This trait is automatically implemented for all types that implement"] # [doc = " [`ToTokens`] from the `quote` crate, as well as for `Span` itself."] # [doc = ""] # [doc = " [`ToTokens`]: quote::ToTokens"] # [doc = ""] # [doc = " See the [module documentation] for an example."] # [doc = ""] # [doc = " [module documentation]: self"] pub trait Spanned : private :: Sealed { # [doc = " Returns a `Span` covering the complete contents of this syntax tree"] # [doc = " node, or [`Span::call_site()`] if this node is empty."] # [doc = ""] # [doc = " [`Span::call_site()`]: proc_macro2::Span::call_site"] fn span (& self) -> Span ; }
    };
}

Spanned!()