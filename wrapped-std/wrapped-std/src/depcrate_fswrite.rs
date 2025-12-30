// Generated macro for write (function)
macro_rules! Depcrate_fswrite {
() => {
// Module: crate::fs
// Provides: {"write"}
// Dependencies: {}
# [doc = " Writes a slice as the entire contents of a file."] # [doc = ""] # [doc = " This function will create a file if it does not exist,"] # [doc = " and will entirely replace its contents if it does."] # [doc = ""] # [doc = " Depending on the platform, this function may fail if the"] # [doc = " full directory path does not exist."] # [doc = ""] # [doc = " This is a convenience function for using [`File::create`] and [`write_all`]"] # [doc = " with fewer imports."] # [doc = ""] # [doc = " [`write_all`]: Write::write_all"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::fs;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     fs::write(\"foo.txt\", b\"Lorem ipsum\")?;"] # [doc = "     fs::write(\"bar.txt\", \"dolor sit\")?;"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "fs_read_write_bytes" , since = "1.26.0")] pub fn write < P : AsRef < Path > , C : AsRef < [u8] > > (path : P , contents : C) -> io :: Result < () > { fn inner (path : & Path , contents : & [u8]) -> io :: Result < () > { File :: create (path) ? . write_all (contents) } inner (path . as_ref () , contents . as_ref ()) }
};
}
