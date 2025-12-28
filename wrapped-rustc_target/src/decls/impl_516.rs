macro_rules! impl_516 {
    () => {
        impl FramePointer { # [doc = " It is intended that the \"force frame pointer\" transition is \"one way\""] # [doc = " so this convenience assures such if used"] # [inline] pub fn ratchet (& mut self , rhs : FramePointer) -> FramePointer { * self = match (* self , rhs) { (FramePointer :: Always , _) | (_ , FramePointer :: Always) => FramePointer :: Always , (FramePointer :: NonLeaf , _) | (_ , FramePointer :: NonLeaf) => FramePointer :: NonLeaf , _ => FramePointer :: MayOmit , } ; * self } }
    };
}

impl_516!();