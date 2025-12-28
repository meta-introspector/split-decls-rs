macro_rules! deps {
    () => {
        Header!();
    };
}

macro_rules! ThinVec {
    () => {
        deps!();
        # [doc = " See the crate's top level documentation for a description of this type."] # [repr (C)] pub struct ThinVec < T > { ptr : NonNull < Header > , boo : PhantomData < T > , }
    };
}

ThinVec!()