macro_rules! ItemKind {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum ItemKind < 'tcx > { Struct (PhantomData < & 'tcx () >) , Enum (PhantomData < & 'tcx () >) , Union (PhantomData < & 'tcx () >) , }
    };
}

ItemKind!()