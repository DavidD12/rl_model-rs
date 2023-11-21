skillset S {
    resource R {
        state {A B}
        initial A
        transition {
            A -> B
        }
    }

    skill S {
        precondition p: R == B
    }
}