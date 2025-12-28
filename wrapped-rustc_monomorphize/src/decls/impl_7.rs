macro_rules! deps {
    () => {
        MonoItems!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < 'tcx > IntoIterator for MonoItems < 'tcx > { type Item = Spanned < MonoItem < 'tcx > > ; type IntoIter = impl Iterator < Item = Spanned < MonoItem < 'tcx > > > ; fn into_iter (self) -> Self :: IntoIter { self . items . into_iter () . map (| (item , span) | respan (span , item)) } }
    };
}

impl_7!();