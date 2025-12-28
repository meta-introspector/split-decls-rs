macro_rules! MaybeDangling {
    () => {
        # [doc = " A wrapper type that tells the compiler that the contents might not be valid."] # [doc = ""] # [doc = " This is necessary mainly when `T` contains a reference. In that case, the"] # [doc = " compiler will sometimes assume that the reference is always valid; in some"] # [doc = " cases it will assume this even after the destructor of `T` runs. For"] # [doc = " example, when a reference is used as a function argument, then the compiler"] # [doc = " will assume that the reference is valid until the function returns, even if"] # [doc = " the reference is destroyed during the function. When the reference is used"] # [doc = " as part of a self-referential struct, that assumption can be false. Wrapping"] # [doc = " the reference in this type prevents the compiler from making that"] # [doc = " assumption."] # [doc = ""] # [doc = " # Invariants"] # [doc = ""] # [doc = " The `MaybeUninit` will always contain a valid value until the destructor runs."] # [repr (transparent)] pub (crate) struct MaybeDangling < T > (MaybeUninit < T >) ;
    };
}

MaybeDangling!();