macro_rules! deps {
    () => {
        Height!();
        Width!();
    };
}

macro_rules! terminal_size {
    () => {
        deps!();
        # [cfg (not (any (unix , windows)))] pub fn terminal_size () -> Option < (Width , Height) > { None }
    };
}

terminal_size!();