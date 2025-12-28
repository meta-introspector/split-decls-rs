macro_rules! deps {
    () => {
        SerializationStrategy!();
        Value!();
        Table!();
        ArrayWalkValue!();
    };
}

macro_rules! impl_301 {
    () => {
        deps!();
        impl ArrayWalkValue { fn new () -> Self { Self { is_empty : true } } fn serialize_element < T > (& mut self , value : & T) -> Result < () , SerializationStrategy > where T : serde_core :: ser :: Serialize + ? Sized , { self . is_empty = false ; match SerializationStrategy :: from (value) { SerializationStrategy :: Value | SerializationStrategy :: ArrayOfTables | SerializationStrategy :: Unknown | SerializationStrategy :: Skip => Err (SerializationStrategy :: Value) , SerializationStrategy :: Table => Ok (()) , } } fn end (self) -> Result < core :: convert :: Infallible , SerializationStrategy > { if self . is_empty { Err (SerializationStrategy :: Value) } else { Err (SerializationStrategy :: ArrayOfTables) } } }
    };
}

impl_301!();