macro_rules! deps {
    () => {
        DropFlagState!();
        Analysis!();
        MaybeUninitializedPlaces!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl < 'tcx > MaybeUninitializedPlaces < '_ , 'tcx > { fn update_bits (state : & mut < Self as Analysis < 'tcx > > :: Domain , path : MovePathIndex , dfstate : DropFlagState ,) { match dfstate { DropFlagState :: Absent => state . gen_ (path) , DropFlagState :: Present => state . kill (path) , } } }
    };
}

impl_145!();