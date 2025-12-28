macro_rules! Tag {
    () => {
        # [doc = " Tag byte used by the `Elliptic-Curve-Point-to-Octet-String` encoding."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [repr (u8)] pub enum Tag { # [doc = " Identity point (`0x00`)"] Identity = 0 , # [doc = " Compressed point with even y-coordinate (`0x02`)"] CompressedEvenY = 2 , # [doc = " Compressed point with odd y-coordinate (`0x03`)"] CompressedOddY = 3 , # [doc = " Uncompressed point (`0x04`)"] Uncompressed = 4 , # [doc = " Compact point (`0x05`)"] Compact = 5 , }
    };
}

Tag!();