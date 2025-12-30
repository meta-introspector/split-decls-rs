// Generated macro for TzifData (struct)
macro_rules! Depcrate_data_tzifTzifData {
() => {
// Module: crate::data::tzif
// Provides: {"TzifData"}
// Dependencies: {}
# [doc = " A struct containing the data of a `TZif` file."] # [doc = " > A `TZif` file is structured as follows:"] # [doc = " > ```text"] # [doc = " >      Version 1       Versions 2 & 3"] # [doc = " >   +-------------+   +-------------+"] # [doc = " >   |  Version 1  |   |  Version 1  |"] # [doc = " >   |   Header    |   |   Header    |"] # [doc = " >   +-------------+   +-------------+"] # [doc = " >   |  Version 1  |   |  Version 1  |"] # [doc = " >   |  Data Block |   |  Data Block |"] # [doc = " >   +-------------+   +-------------+"] # [doc = " >                     |  Version 2+ |"] # [doc = " >                     |   Header    |"] # [doc = " >                     +-------------+"] # [doc = " >                     |  Version 2+ |"] # [doc = " >                     |  Data Block |"] # [doc = " >                     +-------------+"] # [doc = " >                     |   Footer    |"] # [doc = " >                     +-------------+"] # [doc = " > ```"] # [derive (Debug)] pub struct TzifData { # [doc = " The version-1 header, which is always present."] pub header1 : TzifHeader , # [doc = " The version-1 data block, which is always present."] pub data_block1 : DataBlock , # [doc = " The version-2+ header, which is present only in version 2 and 3 `TZif` files."] pub header2 : Option < TzifHeader > , # [doc = " The vesrion-2+ data block, which is present only in version 2 and 3 `TZif` files."] pub data_block2 : Option < DataBlock > , # [doc = " The version-2+ footer, which is present only in version 2 and 3 `TZif` files."] pub footer : Option < PosixTzString > , }
};
}
