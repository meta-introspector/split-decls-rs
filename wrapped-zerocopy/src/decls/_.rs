macro_rules! deps {
    () => {
        KnownLayout!();
        MaybeUninit!();
    };
}

macro_rules! _ {
    () => {
        deps!();
        const _ : () = unsafe { unsafe_impl_known_layout ! (T : ? Sized + KnownLayout => # [repr (T :: MaybeUninit)] MaybeUninit < T >) } ;
    };
}

_!();