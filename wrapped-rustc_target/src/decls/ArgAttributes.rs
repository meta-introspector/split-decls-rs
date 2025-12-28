macro_rules! deps {
    () => {
        ArgExtension!();
    };
}

macro_rules! ArgAttributes {
    () => {
        deps!();
        # [doc = " A compact representation of LLVM attributes (at least those relevant for this module)"] # [doc = " that can be manipulated without interacting with LLVM's Attribute machinery."] # [derive (Copy , Clone , PartialEq , Eq , Hash , Debug , HashStable_Generic)] pub struct ArgAttributes { pub regular : ArgAttribute , pub arg_ext : ArgExtension , # [doc = " The minimum size of the pointee, guaranteed to be valid for the duration of the whole call"] # [doc = " (corresponding to LLVM's dereferenceable_or_null attributes, i.e., it is okay for this to be"] # [doc = " set on a null pointer, but all non-null pointers must be dereferenceable)."] pub pointee_size : Size , # [doc = " The minimum alignment of the pointee, if any."] pub pointee_align : Option < Align > , }
    };
}

ArgAttributes!()