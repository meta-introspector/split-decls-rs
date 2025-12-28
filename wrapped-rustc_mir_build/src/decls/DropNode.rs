macro_rules! deps {
    () => {
        DropData!();
    };
}

macro_rules! DropNode {
    () => {
        deps!();
        # [doc = " A single node in the drop tree."] # [derive (Debug)] struct DropNode { # [doc = " Info about the drop to be performed at this node in the drop tree."] data : DropData , # [doc = " Index of the \"next\" drop to perform (in drop order, not declaration order)."] next : DropIdx , }
    };
}

DropNode!()