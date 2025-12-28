macro_rules! deps {
    () => {
        ModulusSize!();
    };
}

macro_rules! Coordinates {
    () => {
        deps!();
        # [doc = " Enum representing the coordinates of either compressed or uncompressed"] # [doc = " SEC1-encoded elliptic curve points."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub enum Coordinates < 'a , Size : ModulusSize > { # [doc = " Identity point (a.k.a. point at infinity)"] Identity , # [doc = " Compact curve point"] Compact { # [doc = " x-coordinate"] x : & 'a Array < u8 , Size > , } , # [doc = " Compressed curve point"] Compressed { # [doc = " x-coordinate"] x : & 'a Array < u8 , Size > , # [doc = " Is the y-coordinate odd?"] y_is_odd : bool , } , # [doc = " Uncompressed curve point"] Uncompressed { # [doc = " x-coordinate"] x : & 'a Array < u8 , Size > , # [doc = " y-coordinate"] y : & 'a Array < u8 , Size > , } , }
    };
}

Coordinates!()