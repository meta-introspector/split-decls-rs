macro_rules! deps {
    () => {
        ModulusSize!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < T > ModulusSize for T where T : 'static + ArraySize + Copy + Debug , T : Add < U1 , Output : 'static + ArraySize + Copy + Debug > , T : Add < T , Output : 'static + ArraySize + Copy + Debug + Sub < T , Output = T > > , < T as Add < U1 > > :: Output : Add < T , Output : 'static + ArraySize + Copy + Debug > , { type CompressedPointSize = < T as Add < U1 > > :: Output ; type UncompressedPointSize = < Self :: CompressedPointSize as Add < T > > :: Output ; type UntaggedPointSize = < T as Add < T > > :: Output ; }
    };
}

impl_3!();