macro_rules! deps {
    () => {
        ConstValue!();
        DeserializerError!();
    };
}

macro_rules! from_value {
    () => {
        deps!();
        # [doc = " Interpret a `ConstValue` as an instance of type `T`."] # [inline] pub fn from_value < T : DeserializeOwned > (value : ConstValue) -> Result < T , DeserializerError > { T :: deserialize (value) }
    };
}

from_value!();