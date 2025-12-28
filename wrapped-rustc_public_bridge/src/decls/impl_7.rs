macro_rules! deps {
    () => {
        IndexMap!();
        Bridge!();
        Tables!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < 'tcx , B : Bridge > Default for Tables < 'tcx , B > { fn default () -> Self { Self { def_ids : IndexMap :: default () , alloc_ids : IndexMap :: default () , spans : IndexMap :: default () , types : IndexMap :: default () , instances : IndexMap :: default () , ty_consts : IndexMap :: default () , mir_consts : IndexMap :: default () , layouts : IndexMap :: default () , } } }
    };
}

impl_7!()