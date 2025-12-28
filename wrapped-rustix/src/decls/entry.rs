macro_rules! entry {
    () => {
        # [doc = " `getauxval(AT_ENTRY)`—Returns the address of the program entrypoint."] # [doc = ""] # [doc = " Most code interested in the program entrypoint address should instead use a"] # [doc = " symbol reference to `_start`. That will be properly PC-relative or"] # [doc = " relocated if needed, and will come with appropriate pointer type and"] # [doc = " pointer provenance."] # [doc = ""] # [doc = " This function is intended only for use in code that implements those"] # [doc = " relocations, to compute the ASLR offset. It has type `usize`, so it doesn't"] # [doc = " carry any provenance, and it shouldn't be used to dereference memory."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man3/getauxval.3.html"] # [inline] pub fn entry () -> usize { backend :: param :: auxv :: entry () }
    };
}

entry!();