macro_rules! deps {
    () => {
        AncillaryIter!();
    };
}

macro_rules! impl_592 {
    () => {
        deps!();
        impl < 'data , T > AncillaryIter < 'data , T > { # [doc = " Create a new iterator over data in an ancillary buffer."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The buffer must contain valid ancillary data."] unsafe fn new (data : & 'data mut [u8]) -> Self { assert_eq ! (data . len () % size_of ::< T > () , 0) ; Self { data , _marker : PhantomData , } } }
    };
}

impl_592!();