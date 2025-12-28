macro_rules! deps {
    () => {
        ReadFrameHeaderError!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl StdError for ReadFrameHeaderError { fn source (& self) -> Option < & (dyn StdError + 'static) > { match self { ReadFrameHeaderError :: MagicNumberReadError (source) => Some (source) , ReadFrameHeaderError :: FrameDescriptorReadError (source) => Some (source) , ReadFrameHeaderError :: InvalidFrameDescriptor (source) => Some (source) , ReadFrameHeaderError :: WindowDescriptorReadError (source) => Some (source) , ReadFrameHeaderError :: DictionaryIdReadError (source) => Some (source) , ReadFrameHeaderError :: FrameContentSizeReadError (source) => Some (source) , _ => None , } } }
    };
}

impl_34!()