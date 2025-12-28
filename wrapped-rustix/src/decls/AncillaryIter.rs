macro_rules! AncillaryIter {
    () => {
        # [doc = " An iterator over data in an ancillary buffer."] pub struct AncillaryIter < 'data , T > { # [doc = " The data we're iterating over."] data : & 'data mut [u8] , # [doc = " The raw data we're removing."] _marker : PhantomData < T > , }
    };
}

AncillaryIter!()