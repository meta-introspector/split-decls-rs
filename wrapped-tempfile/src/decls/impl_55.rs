macro_rules! deps {
    () => {
        NamedTempFile!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl Write for & NamedTempFile < File > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . as_file () . write (buf) . with_err_path (| | self . path ()) } # [inline] fn flush (& mut self) -> io :: Result < () > { self . as_file () . flush () . with_err_path (| | self . path ()) } fn write_vectored (& mut self , bufs : & [io :: IoSlice < '_ >]) -> io :: Result < usize > { self . as_file () . write_vectored (bufs) . with_err_path (| | self . path ()) } fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { self . as_file () . write_all (buf) . with_err_path (| | self . path ()) } fn write_fmt (& mut self , fmt : fmt :: Arguments < '_ >) -> io :: Result < () > { self . as_file () . write_fmt (fmt) . with_err_path (| | self . path ()) } }
    };
}

impl_55!()