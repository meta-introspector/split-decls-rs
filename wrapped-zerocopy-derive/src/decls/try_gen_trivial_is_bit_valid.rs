macro_rules! deps {
    () => {
        Trait!();
    };
}

macro_rules! try_gen_trivial_is_bit_valid {
    () => {
        deps!();
        # [doc = " Attempts to generate a `TryFromBytes::is_bit_valid` instance that"] # [doc = " unconditionally returns true."] # [doc = ""] # [doc = " This is possible when the `top_level` trait is `FromBytes` and there are no"] # [doc = " generic type parameters. In this case, we know that compilation will succeed"] # [doc = " only if the type is unconditionally `FromBytes`. Type parameters are not"] # [doc = " supported because a type with type parameters could be `TryFromBytes` but"] # [doc = " not `FromBytes` depending on its type parameters, and so deriving a trivial"] # [doc = " `is_bit_valid` would be either unsound or, assuming we add a defensive"] # [doc = " `Self: FromBytes` bound (as we currently do), overly restrictive. Consider,"] # [doc = " for example, that `Foo<bool>` ought to be `TryFromBytes` but not `FromBytes`"] # [doc = " in this example:"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " #[derive(FromBytes)]"] # [doc = " #[repr(transparent)]"] # [doc = " struct Foo<T>(T);"] # [doc = " ```"] # [doc = ""] # [doc = " This should be used where possible. Using this impl is faster to codegen,"] # [doc = " faster to compile, and is friendlier on the optimizer."] fn try_gen_trivial_is_bit_valid (ast : & DeriveInput , top_level : Trait , zerocopy_crate : & Path ,) -> Option < proc_macro2 :: TokenStream > { if top_level == Trait :: FromBytes && ast . generics . params . is_empty () { Some (quote ! (fn is_bit_valid < ___ZerocopyAliasing > (_candidate : # zerocopy_crate :: Maybe < Self , ___ZerocopyAliasing >,) -> # zerocopy_crate :: util :: macro_util :: core_reexport :: primitive :: bool where ___ZerocopyAliasing : # zerocopy_crate :: pointer :: invariant :: Reference , { if false { fn assert_is_from_bytes < T > () where T : # zerocopy_crate :: FromBytes , T : ?# zerocopy_crate :: util :: macro_util :: core_reexport :: marker :: Sized , { } assert_is_from_bytes ::< Self > () ; } true })) } else { None } }
    };
}

try_gen_trivial_is_bit_valid!();