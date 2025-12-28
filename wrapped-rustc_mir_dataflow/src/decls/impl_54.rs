macro_rules! deps {
    () => {
        HasMoveData!();
        DebugWithContext!();
        Formatter!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < 'tcx , C > DebugWithContext < C > for crate :: move_paths :: MovePathIndex where C : crate :: move_paths :: HasMoveData < 'tcx > , { fn fmt_with (& self , ctxt : & C , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , ctxt . move_data () . move_paths [* self]) } }
    };
}

impl_54!();