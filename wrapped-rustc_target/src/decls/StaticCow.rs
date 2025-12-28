macro_rules! StaticCow {
    () => {
        type StaticCow < T > = Cow < 'static , T > ;
    };
}

StaticCow!()