macro_rules! GuardFrameLocal {
    () => {
        # [derive (Debug)] struct GuardFrameLocal { id : LocalVarId , }
    };
}

GuardFrameLocal!()