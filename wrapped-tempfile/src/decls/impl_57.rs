macro_rules! deps {
    () => {
        NamedTempFile!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl Seek for & NamedTempFile < File > { fn seek (& mut self , pos : SeekFrom) -> io :: Result < u64 > { self . as_file () . seek (pos) . with_err_path (| | self . path ()) } }
    };
}

impl_57!();