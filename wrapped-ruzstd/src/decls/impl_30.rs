macro_rules! deps {
    () => {
        FrameHeaderError!();
        FrameDescriptorError!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl StdError for FrameHeaderError { fn source (& self) -> Option < & (dyn StdError + 'static) > { match self { FrameHeaderError :: FrameDescriptorError (source) => Some (source) , _ => None , } } }
    };
}

impl_30!()