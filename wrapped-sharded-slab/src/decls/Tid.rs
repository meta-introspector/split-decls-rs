macro_rules! Tid {
    () => {
        # [doc = " Uniquely identifies a thread."] pub (crate) struct Tid < C > { id : usize , _not_send : PhantomData < UnsafeCell < () > > , _cfg : PhantomData < fn (C) > , }
    };
}

Tid!()