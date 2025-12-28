macro_rules! impl_255 {
    () => {
        impl < W : Write + Seek > Drop for ZipWriter < W > { fn drop (& mut self) { if ! self . inner . is_closed () { if let Err (e) = self . finalize () { let _ = write ! (io :: stderr () , "ZipWriter drop failed: {e:?}") ; } } } }
    };
}

impl_255!()