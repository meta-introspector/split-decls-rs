macro_rules! deps {
    () => {
        NoneLayerMarker!();
    };
}

macro_rules! NONE_LAYER_MARKER {
    () => {
        deps!();
        static NONE_LAYER_MARKER : NoneLayerMarker = NoneLayerMarker (()) ;
    };
}

NONE_LAYER_MARKER!();