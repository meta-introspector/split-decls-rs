macro_rules! deps {
    () => {
        HSTRING!();
        HStringHeader!();
    };
}

macro_rules! HStringBuilder {
    () => {
        deps!();
        # [doc = " An [HSTRING] builder that supports preallocating the `HSTRING` to avoid extra allocations and copies."] # [doc = ""] # [doc = " This is similar to the `WindowsPreallocateStringBuffer` function but implemented directly in Rust for efficiency."] # [doc = " It is implemented as a separate type since [HSTRING] values are immutable."] pub struct HStringBuilder (* mut HStringHeader) ;
    };
}

HStringBuilder!()