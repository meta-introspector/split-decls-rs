macro_rules! block_on_future {
    () => {
        pub fn block_on_future < F > (future : F) -> F :: Output where F : std :: future :: Future + Send + 'static , F :: Output : Send + 'static , { tokio_test :: block_on (future) }
    };
}

block_on_future!()