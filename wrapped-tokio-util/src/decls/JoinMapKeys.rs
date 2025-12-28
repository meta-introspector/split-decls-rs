macro_rules! deps {
    () => {
        JoinMap!();
    };
}

macro_rules! JoinMapKeys {
    () => {
        deps!();
        # [doc = " An iterator over the keys of a [`JoinMap`]."] # [derive (Debug , Clone)] pub struct JoinMapKeys < 'a , K , V > { iter : hashbrown :: hash_table :: Iter < 'a , (K , AbortHandle) > , # [doc = " To make it easier to change `JoinMap` in the future, keep V as a generic"] # [doc = " parameter."] _value : PhantomData < & 'a V > , }
    };
}

JoinMapKeys!();