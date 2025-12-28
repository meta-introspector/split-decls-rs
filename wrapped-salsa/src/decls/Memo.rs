macro_rules! Memo {
    () => {
        pub type Memo < C > = memo :: Memo < 'static , C > ;
    };
}

Memo!()