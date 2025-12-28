macro_rules! MovePathResult {
    () => {
        enum MovePathResult { Path (MovePathIndex) , Union (MovePathIndex) , Error , }
    };
}

MovePathResult!()