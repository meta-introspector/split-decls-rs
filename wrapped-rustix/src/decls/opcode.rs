macro_rules! deps {
    () => {
        Direction!();
        Opcode!();
    };
}

macro_rules! opcode {
    () => {
        deps!();
        # [doc = " Const functions for computing opcode values."] # [doc = ""] # [doc = " Linux's headers define macros such as `_IO`, `_IOR`, `_IOW`, and `_IOWR`"] # [doc = " for defining ioctl values in a structured way that encode whether they"] # [doc = " are reading and/or writing, and other information about the ioctl. The"] # [doc = " functions in this module correspond to those macros."] # [doc = ""] # [doc = " If you're writing a driver and defining your own ioctl numbers, it's"] # [doc = " recommended to use these functions to compute them."] # [cfg (any (linux_kernel , bsd))] pub mod opcode { use super :: * ; # [doc = " Create a new opcode from a direction, group, number, and size."] # [doc = ""] # [doc = " This corresponds to the C macro `_IOC(direction, group, number, size)`"] # [doc (alias = "_IOC")] # [inline] pub const fn from_components (direction : Direction , group : u8 , number : u8 , data_size : usize ,) -> Opcode { assert ! (data_size <= Opcode :: MAX as usize , "data size is too large") ; platform :: compose_opcode (direction , group as Opcode , number as Opcode , data_size as Opcode ,) } # [doc = " Create a new opcode from a group, a number, that uses no data."] # [doc = ""] # [doc = " This corresponds to the C macro `_IO(group, number)`."] # [doc (alias = "_IO")] # [inline] pub const fn none (group : u8 , number : u8) -> Opcode { from_components (Direction :: None , group , number , 0) } # [doc = " Create a new reading opcode from a group, a number and the type of"] # [doc = " data."] # [doc = ""] # [doc = " This corresponds to the C macro `_IOR(group, number, T)`."] # [doc (alias = "_IOR")] # [inline] pub const fn read < T > (group : u8 , number : u8) -> Opcode { from_components (Direction :: Read , group , number , mem :: size_of :: < T > ()) } # [doc = " Create a new writing opcode from a group, a number and the type of"] # [doc = " data."] # [doc = ""] # [doc = " This corresponds to the C macro `_IOW(group, number, T)`."] # [doc (alias = "_IOW")] # [inline] pub const fn write < T > (group : u8 , number : u8) -> Opcode { from_components (Direction :: Write , group , number , mem :: size_of :: < T > ()) } # [doc = " Create a new reading and writing opcode from a group, a number and the"] # [doc = " type of data."] # [doc = ""] # [doc = " This corresponds to the C macro `_IOWR(group, number, T)`."] # [doc (alias = "_IOWR")] # [inline] pub const fn read_write < T > (group : u8 , number : u8) -> Opcode { from_components (Direction :: ReadWrite , group , number , mem :: size_of :: < T > ()) } }
    };
}

opcode!();