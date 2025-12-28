macro_rules! deps {
    () => {
        Path!();
    };
}

macro_rules! MapAccess {
    () => {
        deps!();
        # [doc = " Map visitor that captures the string value of its keys and uses that to"] # [doc = " track the path to its values."] struct MapAccess < 'a , 'b , X , F : 'b > { delegate : X , callback : & 'b mut F , path : & 'a Path < 'a > , key : Option < String > , }
    };
}

MapAccess!();