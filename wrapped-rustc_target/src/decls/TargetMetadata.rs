macro_rules! deps {
    () => {
        StaticCow!();
    };
}

macro_rules! TargetMetadata {
    () => {
        deps!();
        # [doc = " Metadata about a target like the description or tier."] # [doc = " Part of #120745."] # [doc = " All fields are optional for now, but intended to be required in the future."] # [derive (Default , PartialEq , Clone , Debug)] pub struct TargetMetadata { # [doc = " A short description of the target including platform requirements,"] # [doc = " for example \"64-bit Linux (kernel 3.2+, glibc 2.17+)\"."] pub description : Option < StaticCow < str > > , # [doc = " The tier of the target. 1, 2 or 3."] pub tier : Option < u64 > , # [doc = " Whether the Rust project ships host tools for a target."] pub host_tools : Option < bool > , # [doc = " Whether a target has the `std` library. This is usually true for targets running"] # [doc = " on an operating system."] pub std : Option < bool > , }
    };
}

TargetMetadata!()