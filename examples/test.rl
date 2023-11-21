type T

skillset S {

    resource R {
        state { On Off }
        initial Off
        transition {
            Off -> On
        }
    }

/*
    event E {
        guard R==On
        effect R -> Off
    }
*/
}