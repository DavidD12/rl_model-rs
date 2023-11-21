type {
    Float
    Double
}

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
        input x: Float
        output {
            y: Float
            z: Double
        }
    }
}