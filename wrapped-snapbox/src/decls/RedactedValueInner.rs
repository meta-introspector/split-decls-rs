macro_rules! RedactedValueInner {
    () => {
        # [derive (Clone , Debug)] enum RedactedValueInner { Str (& 'static str) , String (String) , Path { native : String , normalized : String , } , # [cfg (feature = "regex")] Regex (regex :: Regex) , }
    };
}

RedactedValueInner!();