macro_rules! deps {
    () => {
        ZipFileReader!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl < 'a , R : Read > ZipFileReader < 'a , R > { fn into_inner (self) -> io :: Result < io :: Take < & 'a mut R > > { match self { ZipFileReader :: NoReader => invalid_state () , ZipFileReader :: Raw (r) => Ok (r) , ZipFileReader :: Compressed (r) => { Ok (r . into_inner () . into_inner () ? . into_inner () . into_inner ()) } } } }
    };
}

impl_86!();