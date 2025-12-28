macro_rules! deps {
    () => {
        ZipFile!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl < R : Read > Drop for ZipFile < '_ , R > { fn drop (& mut self) { if let Cow :: Owned (_) = self . data { if let Ok (mut inner) = self . take_raw_reader () { let _ = copy (& mut inner , & mut sink ()) ; } } } }
    };
}

impl_122!()