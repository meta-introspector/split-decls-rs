macro_rules! deps {
    () => {
        ABI!();
        ArgAbi!();
    };
}

macro_rules! FnAbi {
    () => {
        deps!();
        # [doc = " Metadata describing how the arguments to a native function"] # [doc = " should be passed in order to respect the native ABI."] # [doc = ""] # [doc = " The signature represented by this type may not match the MIR function signature."] # [doc = " Certain attributes, like `#[track_caller]` can introduce additional arguments, which are present in [`FnAbi`], but not in `FnSig`."] # [doc = " While this difference is rarely relevant, it should still be kept in mind."] # [doc = ""] # [doc = " I will do my best to describe this structure, but these"] # [doc = " comments are reverse-engineered and may be inaccurate. -NDM"] # [derive (Clone , PartialEq , Eq , Hash , HashStable_Generic)] pub struct FnAbi < 'a , Ty > { # [doc = " The type, layout, and information about how each argument is passed."] pub args : Box < [ArgAbi < 'a , Ty >] > , # [doc = " The layout, type, and the way a value is returned from this function."] pub ret : ArgAbi < 'a , Ty > , # [doc = " Marks this function as variadic (accepting a variable number of arguments)."] pub c_variadic : bool , # [doc = " The count of non-variadic arguments."] # [doc = ""] # [doc = " Should only be different from args.len() when c_variadic is true."] # [doc = " This can be used to know whether an argument is variadic or not."] pub fixed_count : u32 , # [doc = " The calling convention of this function."] pub conv : CanonAbi , # [doc = " Indicates if an unwind may happen across a call to this function."] pub can_unwind : bool , }
    };
}

FnAbi!();