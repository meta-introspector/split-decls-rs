macro_rules! Entry {
    () => {
        # [doc = " A single entry in the table contains the decoded symbol/literal and the"] # [doc = " size of the prefix code."] # [derive (Copy , Clone , Debug)] pub struct Entry { # [doc = " The byte that the prefix code replaces during encoding."] symbol : u8 , # [doc = " The number of bits the prefix code occupies."] num_bits : u8 , }
    };
}

Entry!()