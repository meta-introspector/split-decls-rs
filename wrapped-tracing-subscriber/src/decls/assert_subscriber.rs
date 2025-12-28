macro_rules! assert_subscriber {
    () => {
        fn assert_subscriber (_s : impl Subscriber) { }
    };
}

assert_subscriber!()