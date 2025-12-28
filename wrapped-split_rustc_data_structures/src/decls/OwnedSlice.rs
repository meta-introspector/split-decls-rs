macro_rules! OwnedSlice {
    () => {
        # [doc = " An owned slice."] # [doc = ""] # [doc = " This is similar to `Arc<[u8]>` but allows slicing and using anything as the"] # [doc = " backing buffer."] # [doc = ""] # [doc = " See [`slice_owned`] for `OwnedSlice` construction and examples."] # [doc = ""] # [doc = " ---------------------------------------------------------------------------"] # [doc = ""] # [doc = " This is essentially a replacement for `owning_ref` which is a lot simpler"] # [doc = " and even sound! 🌸"] # [derive (Clone)] pub struct OwnedSlice { # [doc = " This is conceptually a `&'self.owner [u8]`."] bytes : * const [u8] , # [expect (dead_code)] owner : Arc < dyn Send + Sync > , }
    };
}

OwnedSlice!()