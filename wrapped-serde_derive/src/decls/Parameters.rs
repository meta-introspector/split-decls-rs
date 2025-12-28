macro_rules! Parameters {
    () => {
        struct Parameters { # [doc = " Variable holding the value being serialized. Either `self` for local"] # [doc = " types or `__self` for remote types."] self_var : Ident , # [doc = " Path to the type the impl is for. Either a single `Ident` for local"] # [doc = " types (does not include generic parameters) or `some::remote::Path` for"] # [doc = " remote types."] this_type : syn :: Path , # [doc = " Same as `this_type` but using `::<T>` for generic parameters for use in"] # [doc = " expression position."] this_value : syn :: Path , # [doc = " Generics including any explicit and inferred bounds for the impl."] generics : syn :: Generics , # [doc = " Type has a `serde(remote = \"...\")` attribute."] is_remote : bool , # [doc = " Type has a repr(packed) attribute."] is_packed : bool , }
    };
}

Parameters!()