macro_rules! deps {
    () => {
        MonoItems!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < 'tcx > Extend < Spanned < MonoItem < 'tcx > > > for MonoItems < 'tcx > { fn extend < I > (& mut self , iter : I) where I : IntoIterator < Item = Spanned < MonoItem < 'tcx > > > , { for item in iter { self . push (item) } } }
    };
}

impl_8!()