macro_rules! DepNodeColor {
    () => {
        # [derive (Debug)] pub (super) enum DepNodeColor { Red , Green (DepNodeIndex) , }
    };
}

DepNodeColor!();