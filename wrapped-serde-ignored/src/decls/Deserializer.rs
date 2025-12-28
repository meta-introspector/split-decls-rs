macro_rules! deps {
    () => {
        Path!();
    };
}

macro_rules! Deserializer {
    () => {
        deps!();
        # [doc = " Deserializer adapter that invokes a callback with the path to every unused"] # [doc = " field of the input."] pub struct Deserializer < 'a , 'b , D , F : 'b > { de : D , callback : & 'b mut F , path : Path < 'a > , }
    };
}

Deserializer!()