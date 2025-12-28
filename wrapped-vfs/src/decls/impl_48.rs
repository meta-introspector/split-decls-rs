macro_rules! deps {
    () => {
        FileId!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        # [doc = " safe because `FileId` is a newtype of `u32`"] impl nohash_hasher :: IsEnabled for FileId { }
    };
}

impl_48!();