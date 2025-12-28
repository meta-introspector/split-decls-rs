macro_rules! Timestamp {
    () => {
        # [doc = " A timestamp that can be encoded into a UUID."] # [doc = ""] # [doc = " This type abstracts the specific encoding, so versions 1, 6, and 7"] # [doc = " UUIDs can both be supported through the same type, even"] # [doc = " though they have a different representation of a timestamp."] # [doc = ""] # [doc = " # References"] # [doc = ""] # [doc = " * [Timestamp Considerations in RFC 9562](https://www.ietf.org/rfc/rfc9562.html#section-6.1)"] # [doc = " * [UUID Generator States in RFC 9562](https://www.ietf.org/rfc/rfc9562.html#section-6.3)"] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct Timestamp { seconds : u64 , subsec_nanos : u32 , counter : u128 , usable_counter_bits : u8 , }
    };
}

Timestamp!()