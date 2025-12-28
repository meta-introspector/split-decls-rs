macro_rules! deps {
    () => {
        DecompositionType!();
    };
}

macro_rules! Decompositions {
    () => {
        deps!();
        # [doc = " External iterator for a string decomposition's characters."] # [derive (Clone)] pub struct Decompositions < I > { kind : DecompositionType , iter : Fuse < I > , buffer : TinyVec < [(u8 , char) ; 4] > , ready : Range < usize > , }
    };
}

Decompositions!()