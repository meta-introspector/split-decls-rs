macro_rules! deps {
    () => {
        NamedTempFile!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < F : Seek > Seek for NamedTempFile < F > { fn seek (& mut self , pos : SeekFrom) -> io :: Result < u64 > { self . as_file_mut () . seek (pos) . with_err_path (| | self . path ()) } }
    };
}

impl_56!()