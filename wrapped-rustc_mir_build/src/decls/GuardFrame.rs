macro_rules! deps {
    () => {
        GuardFrameLocal!();
    };
}

macro_rules! GuardFrame {
    () => {
        deps!();
        # [derive (Debug)] struct GuardFrame { # [doc = " These are the id's of names that are bound by patterns of the"] # [doc = " arm of *this* guard."] # [doc = ""] # [doc = " (Frames higher up the stack will have the id's bound in arms"] # [doc = " further out, such as in a case like:"] # [doc = ""] # [doc = " match E1 {"] # [doc = "      P1(id1) if (... (match E2 { P2(id2) if ... => B2 })) => B1,"] # [doc = " }"] # [doc = ""] # [doc = " here, when building for FIXME."] locals : Vec < GuardFrameLocal > , }
    };
}

GuardFrame!();