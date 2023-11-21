skillset S {
    resource R {
        state {A B}
        initial A
        transition all
    }

    event E {
        guard R == A or R == B
        effect R -> B
    }
}