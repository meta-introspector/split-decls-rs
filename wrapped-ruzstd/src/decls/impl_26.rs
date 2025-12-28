macro_rules! deps {
    () => {
        FrameDescriptorError!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl fmt :: Display for FrameDescriptorError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: InvalidFrameContentSizeFlag { got } => write ! (f , "Invalid Frame_Content_Size_Flag; Is: {got}, Should be one of: 0, 1, 2, 3") , } } }
    };
}

impl_26!();