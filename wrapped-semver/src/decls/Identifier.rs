macro_rules! Identifier {
    () => {
        # [repr (C , align (8))] pub (crate) struct Identifier { head : NonNull < u8 > , tail : [u8 ; TAIL_BYTES] , }
    };
}

Identifier!();