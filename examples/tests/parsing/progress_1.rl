type Float

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
        progress {
            period 2 sec
            message {
                x: Float
            }
        }
    }
}