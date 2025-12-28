macro_rules! deps {
    () => {
        SnapshotMap!();
    };
}

macro_rules! nested_commit_then_rollback {
    () => {
        deps!();
        # [test] fn nested_commit_then_rollback () { let mut map = SnapshotMap :: default () ; map . insert (22 , "twenty-two") ; let snapshot1 = map . snapshot () ; let snapshot2 = map . snapshot () ; map . insert (22 , "thirty-three") ; map . commit (snapshot2) ; assert_eq ! (map [& 22] , "thirty-three") ; map . rollback_to (snapshot1) ; assert_eq ! (map [& 22] , "twenty-two") ; }
    };
}

nested_commit_then_rollback!();