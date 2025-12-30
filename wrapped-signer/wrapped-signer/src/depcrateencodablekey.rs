// Generated macro for EncodableKey (trait)
macro_rules! DepcrateEncodableKey {
() => {
// Module: crate
// Provides: {"EncodableKey"}
// Dependencies: {}
# [doc = " The `EncodableKey` trait defines the interface by which cryptographic keys/keypairs are read,"] # [doc = " written, and derived from sources."] pub trait EncodableKey : Sized { fn read < R : Read > (reader : & mut R) -> Result < Self , Box < dyn error :: Error > > ; fn read_from_file < F : AsRef < Path > > (path : F) -> Result < Self , Box < dyn error :: Error > > { let mut file = File :: open (path . as_ref ()) ? ; Self :: read (& mut file) } fn write < W : Write > (& self , writer : & mut W) -> Result < String , Box < dyn error :: Error > > ; fn write_to_file < F : AsRef < Path > > (& self , outfile : F) -> Result < String , Box < dyn error :: Error > > { let outfile = outfile . as_ref () ; if let Some (outdir) = outfile . parent () { fs :: create_dir_all (outdir) ? ; } let mut f = { # [cfg (not (unix))] { OpenOptions :: new () } # [cfg (unix)] { use std :: os :: unix :: fs :: OpenOptionsExt ; OpenOptions :: new () . mode (0o600) } } . write (true) . truncate (true) . create (true) . open (outfile) ? ; self . write (& mut f) } }
};
}
