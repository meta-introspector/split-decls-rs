macro_rules! LatchRef {
    () => {
        # [doc = " `&L` without any implication of `dereferenceable` for `Latch::set`"] pub (super) struct LatchRef < 'a , L > { inner : * const L , marker : PhantomData < & 'a L > , }
    };
}

LatchRef!()