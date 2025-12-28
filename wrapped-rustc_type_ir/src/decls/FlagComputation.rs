macro_rules! FlagComputation {
    () => {
        # [derive (Debug)] pub struct FlagComputation < I > { pub flags : TypeFlags , # [doc = " see `Ty::outer_exclusive_binder` for details"] pub outer_exclusive_binder : ty :: DebruijnIndex , interner : std :: marker :: PhantomData < I > , }
    };
}

FlagComputation!()