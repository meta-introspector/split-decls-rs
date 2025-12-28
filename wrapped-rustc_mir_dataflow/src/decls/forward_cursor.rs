macro_rules! deps {
    () => {
        Forward!();
        MockAnalysis!();
    };
}

macro_rules! forward_cursor {
    () => {
        deps!();
        # [test] fn forward_cursor () { let body = mock_body () ; let body = & body ; let analysis = MockAnalysis { body , dir : PhantomData :: < Forward > } ; test_cursor (analysis) }
    };
}

forward_cursor!()