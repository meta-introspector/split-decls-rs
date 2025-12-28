macro_rules! deps {
    () => {
        RecvAncillaryBuffer!();
    };
}

macro_rules! impl_576 {
    () => {
        deps!();
        impl < 'buf > From < & 'buf mut [MaybeUninit < u8 >] > for RecvAncillaryBuffer < 'buf > { fn from (buffer : & 'buf mut [MaybeUninit < u8 >]) -> Self { Self :: new (buffer) } }
    };
}

impl_576!();