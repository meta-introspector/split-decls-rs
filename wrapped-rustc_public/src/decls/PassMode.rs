macro_rules! deps {
    () => {
        Opaque!();
        Scalar!();
    };
}

macro_rules! PassMode {
    () => {
        deps!();
        # [doc = " How a function argument should be passed in to the target function."] # [derive (Clone , Debug , PartialEq , Eq , Hash , Serialize)] pub enum PassMode { # [doc = " Ignore the argument."] # [doc = ""] # [doc = " The argument is either uninhabited or a ZST."] Ignore , # [doc = " Pass the argument directly."] # [doc = ""] # [doc = " The argument has a layout abi of `Scalar` or `Vector`."] Direct (Opaque) , # [doc = " Pass a pair's elements directly in two arguments."] # [doc = ""] # [doc = " The argument has a layout abi of `ScalarPair`."] Pair (Opaque , Opaque) , # [doc = " Pass the argument after casting it."] Cast { pad_i32 : bool , cast : Opaque } , # [doc = " Pass the argument indirectly via a hidden pointer."] Indirect { attrs : Opaque , meta_attrs : Opaque , on_stack : bool } , }
    };
}

PassMode!()