macro_rules! deps {
    () => {
        DropFlagState!();
        Analysis!();
        MaybeInitializedPlaces!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl < 'a , 'tcx > MaybeInitializedPlaces < 'a , 'tcx > { fn update_bits (state : & mut < Self as Analysis < 'tcx > > :: Domain , path : MovePathIndex , dfstate : DropFlagState ,) { match dfstate { DropFlagState :: Absent => state . kill (path) , DropFlagState :: Present => state . gen_ (path) , } } }
    };
}

impl_144!();