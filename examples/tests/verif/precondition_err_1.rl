skillset S {
    resource R {
        state {A B}
        initial A
        transition {
            A -> B
        }
    }

    skill S {
        precondition p1: R == B
        precondition p2: R == A
    }
}