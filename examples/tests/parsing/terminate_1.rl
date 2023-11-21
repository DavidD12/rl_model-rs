skillset Robot {
    resource R {
        state { s1 s2 }
        initial s2
        transition {
            s1 -> s2
            s2 -> s1
        }
    }

    skill S {
        success OK {
            effect R -> s1
            postcondition R == s1
        }
        failure {
            KO {}
            Fail {}
        }
    }
}