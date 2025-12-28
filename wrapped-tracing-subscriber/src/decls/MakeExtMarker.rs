macro_rules! MakeExtMarker {
    () => {
        # [derive (Debug)] # [doc (hidden)] pub struct MakeExtMarker < T > { _p : PhantomData < T > , }
    };
}

MakeExtMarker!()