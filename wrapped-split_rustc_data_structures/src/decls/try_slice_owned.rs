macro_rules! deps {
    () => {
        OwnedSlice!();
    };
}

macro_rules! try_slice_owned {
    () => {
        deps!();
        # [doc = " Makes an [`OwnedSlice`] out of an `owner` and a `slicer` function that can fail."] # [doc = ""] # [doc = " See [`slice_owned`] for the infallible version."] pub fn try_slice_owned < O , F , E > (owner : O , slicer : F) -> Result < OwnedSlice , E > where O : Send + Sync + 'static , F : FnOnce (& O) -> Result < & [u8] , E > , { let owner = Arc :: new (owner) ; let bytes = slicer (& * owner) ? ; Ok (OwnedSlice { bytes , owner }) }
    };
}

try_slice_owned!()