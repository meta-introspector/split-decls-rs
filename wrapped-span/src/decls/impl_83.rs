macro_rules! deps {
    () => {
        EditionedFileId!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl fmt :: Debug for EditionedFileId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("EditionedFileId") . field (& self . file_id () . index ()) . field (& self . edition ()) . finish () } }
    };
}

impl_83!()