macro_rules! deps {
    () => {
        TestCase!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < 'tcx > TestCase < 'tcx > { fn as_range (& self) -> Option < & PatRange < 'tcx > > { if let Self :: Range (v) = self { Some (v . as_ref ()) } else { None } } }
    };
}

impl_118!()