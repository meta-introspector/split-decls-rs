macro_rules! State {
    () => {
        # [derive (Debug)] enum State < T > { Idle (Sender < T >) , Acquiring , ReadyToSend (OwnedPermit < T >) , Closed , }
    };
}

State!()