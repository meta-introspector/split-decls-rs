macro_rules! deps {
    () => {
        Backward!();
        MockAnalysis!();
    };
}

macro_rules! backward_cursor {
    () => {
        deps!();
        # [test] fn backward_cursor () { let body = mock_body () ; let body = & body ; let analysis = MockAnalysis { body , dir : PhantomData :: < Backward > } ; test_cursor (analysis) }
    };
}

backward_cursor!()