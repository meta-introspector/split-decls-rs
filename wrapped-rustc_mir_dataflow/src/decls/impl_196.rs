macro_rules! deps {
    () => {
        LocationMap!();
    };
}

macro_rules! impl_196 {
    () => {
        deps!();
        impl < T > LocationMap < T > where T : Default + Clone , { fn new (body : & Body < '_ >) -> Self { LocationMap { map : body . basic_blocks . iter () . map (| block | vec ! [T :: default () ; block . statements . len () + 1]) . collect () , } } }
    };
}

impl_196!();