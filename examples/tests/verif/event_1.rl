skillset S {
    resource R {
        state {A B}
        initial A
        transition all
    }

    event E {
        guard R == A
        effect R -> B
    }
}