macro_rules! deps {
    () => {
        SendAncillaryBuffer!();
    };
}

macro_rules! impl_571 {
    () => {
        deps!();
        impl < 'buf > From < & 'buf mut [MaybeUninit < u8 >] > for SendAncillaryBuffer < 'buf , '_ , '_ > { fn from (buffer : & 'buf mut [MaybeUninit < u8 >]) -> Self { Self :: new (buffer) } }
    };
}

impl_571!()