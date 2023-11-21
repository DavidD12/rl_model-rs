skillset S {
    resource R {
        state {A B}
        initial A
        transition all
    }

    event E {
        guard R == A and R == B
        effect R -> B
    }
}