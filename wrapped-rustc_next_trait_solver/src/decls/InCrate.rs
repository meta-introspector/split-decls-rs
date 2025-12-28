macro_rules! deps {
    () => {
        OrphanCheckMode!();
    };
}

macro_rules! InCrate {
    () => {
        deps!();
        # [doc = " Whether we do the orphan check relative to this crate or to some remote crate."] # [derive (Copy , Clone , Debug)] pub enum InCrate { Local { mode : OrphanCheckMode } , Remote , }
    };
}

InCrate!()