skillset Robot {
    resource R {
        state { s1 s2 s1 }
        initial s2
        transition {
            s1 -> s2
            s2 -> s1
        }
    }
}