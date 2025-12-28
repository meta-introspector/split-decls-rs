macro_rules! NoneLayerMarker {
    () => {
        # [derive (Clone , Copy)] pub (crate) struct NoneLayerMarker (()) ;
    };
}

NoneLayerMarker!();