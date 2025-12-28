macro_rules! deps {
    () => {
        ConstValue!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < 'a , T : Clone + Into < ConstValue > > From < & 'a [T] > for ConstValue { fn from (f : & 'a [T]) -> Self { ConstValue :: List (f . iter () . cloned () . map (Into :: into) . collect ()) } }
    };
}

impl_34!()