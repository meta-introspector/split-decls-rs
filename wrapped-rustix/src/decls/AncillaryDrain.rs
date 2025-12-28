macro_rules! deps {
    () => {
        RecvAncillaryBuffer!();
    };
}

macro_rules! AncillaryDrain {
    () => {
        deps!();
        # [doc = " An iterator that drains messages from a [`RecvAncillaryBuffer`]."] pub struct AncillaryDrain < 'buf > { # [doc = " Inner iterator over messages."] messages : messages :: Messages < 'buf > , # [doc = " Increment the number of messages we've read."] # [doc = " Decrement the total length."] read_and_length : Option < (& 'buf mut usize , & 'buf mut usize) > , }
    };
}

AncillaryDrain!()