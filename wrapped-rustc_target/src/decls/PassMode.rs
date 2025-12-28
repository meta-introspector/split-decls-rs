macro_rules! deps {
    () => {
        ArgAttributes!();
        ABI!();
        CastTarget!();
    };
}

macro_rules! PassMode {
    () => {
        deps!();
        # [derive (Clone , PartialEq , Eq , Hash , Debug , HashStable_Generic)] pub enum PassMode { # [doc = " Ignore the argument."] # [doc = ""] # [doc = " The argument is a ZST."] Ignore , # [doc = " Pass the argument directly."] # [doc = ""] # [doc = " The argument has a layout abi of `Scalar` or `Vector`."] # [doc = " Unfortunately due to past mistakes, in rare cases on wasm, it can also be `Aggregate`."] # [doc = " This is bad since it leaks LLVM implementation details into the ABI."] # [doc = " (Also see <https://github.com/rust-lang/rust/issues/115666>.)"] Direct (ArgAttributes) , # [doc = " Pass a pair's elements directly in two arguments."] # [doc = ""] # [doc = " The argument has a layout abi of `ScalarPair`."] Pair (ArgAttributes , ArgAttributes) , # [doc = " Pass the argument after casting it. See the `CastTarget` docs for details."] # [doc = ""] # [doc = " `pad_i32` indicates if a `Reg::i32()` dummy argument is emitted before the real argument."] Cast { pad_i32 : bool , cast : Box < CastTarget > } , # [doc = " Pass the argument indirectly via a hidden pointer."] # [doc = ""] # [doc = " The `meta_attrs` value, if any, is for the metadata (vtable or length) of an unsized"] # [doc = " argument. (This is the only mode that supports unsized arguments.)"] # [doc = ""] # [doc = " `on_stack` defines that the value should be passed at a fixed stack offset in accordance to"] # [doc = " the ABI rather than passed using a pointer. This corresponds to the `byval` LLVM argument"] # [doc = " attribute. The `byval` argument will use a byte array with the same size as the Rust type"] # [doc = " (which ensures that padding is preserved and that we do not rely on LLVM's struct layout),"] # [doc = " and will use the alignment specified in `attrs.pointee_align` (if `Some`) or the type's"] # [doc = " alignment (if `None`). This means that the alignment will not always"] # [doc = " match the Rust type's alignment; see documentation of `pass_by_stack_offset` for more info."] # [doc = ""] # [doc = " `on_stack` cannot be true for unsized arguments, i.e., when `meta_attrs` is `Some`."] Indirect { attrs : ArgAttributes , meta_attrs : Option < ArgAttributes > , on_stack : bool } , }
    };
}

PassMode!()