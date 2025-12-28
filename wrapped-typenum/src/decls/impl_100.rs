macro_rules! deps {
    () => {
        NonZero!();
        NInt!();
        Cmp!();
        InternalMarker!();
        Internal!();
        Unsigned!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        # [doc = " -X <==> -Y"] impl < Nl : Unsigned + NonZero , Nr : Cmp < Nl > + Unsigned + NonZero > Cmp < NInt < Nr > > for NInt < Nl > { type Output = < Nr as Cmp < Nl > > :: Output ; # [inline] fn compare < IM : InternalMarker > (& self , rhs : & NInt < Nr >) -> Self :: Output { rhs . n . compare :: < Internal > (& self . n) } }
    };
}

impl_100!();