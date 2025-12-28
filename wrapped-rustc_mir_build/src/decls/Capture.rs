macro_rules! Capture {
    () => {
        # [derive (Debug)] struct Capture < 'tcx > { captured_place : & 'tcx ty :: CapturedPlace < 'tcx > , use_place : Place < 'tcx > , mutability : Mutability , }
    };
}

Capture!();