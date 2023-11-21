type Battery
type GeoPoint
type Float

skillset uav {
    data {
        battery: Battery
        position: GeoPoint period 1 sec
        home: GeoPoint    
    }

    resource authority {
        state { None Pilot Drone }
        initial Pilot
        transition {
            None  -> Drone
            None  -> Pilot
            Drone -> None
            Drone -> Pilot
            Pilot -> None            
        }
    }

    resource flight_status {
        state { NotReady OnGround InAir }
        initial NotReady
        transition all
    }

    resource motion {
        state { Free Used }
        initial Free
        transition all
    }

    resource battery {
        state { Good Low Critical }
        initial Good
        transition {
            Good -> Low
            Good -> Critical
            Low  -> Critical
        }
    }

    event take_authority {
        guard authority != Pilot
        effect authority -> Pilot
    }

    skill takeoff {
        input {
            height: Float // [m] validate can fail if h>h_geo_fence
            speed : Float // [m/s] maximum ascending velocity
        }
        output {
            height: Float // [m] validate can fail if h>h_geo_fence
        }
        precondition {
            has_authority: authority == Drone
            on_ground    : flight_status == OnGround
            not_moving   : motion == Free
            battery_good : battery == Good
        }
        start motion -> Used
        invariant {
            in_control {
                guard motion == Used
            }
            has_authority {
                guard  authority == Drone
                effect motion -> Free
            }
            battery {
                guard battery != Critical
                effect motion -> Free
            }
        }
        progress {
            period  1 sec
            message height: Float
        }
        interrupt {
            interrupting true
            effect motion -> Free
        }
        success at_altitude {
            effect motion -> Free
            postcondition flight_status == InAir
        }
        failure {
            grounded {
                effect motion -> Free
                postcondition flight_status == OnGround
            }
            emergency {
                effect motion -> Free
                postcondition flight_status == InAir
            }
        }
    }

    skill goto {
        precondition {
            has_authority: authority != Pilot
            in_air       : flight_status == InAir
            not_moving   : motion == Free
            battery_good : battery != Critical
        }
        start motion -> Used
        invariant {
            in_control {
                guard motion == Used
            }
            has_authority {
                guard  authority == Drone
                effect motion -> Free
            }
            in_air {
                guard flight_status == InAir
            }
            battery {
                guard battery != Critical
                effect motion -> Free
            }
        }
        success ok {
            effect {
                motion -> Free
            }
        }
        failure ko {
            //...
        }
    }

    skill goto_sol_1 {
        precondition {
            has_authority: authority == Drone
            in_air       : flight_status == InAir
            not_moving   : motion == Free
            battery_good : battery != Critical
        }
        start motion -> Used
        invariant {
            in_control {
                guard motion == Used
            }
            has_authority {
                guard  authority == Drone
                effect motion -> Free
            }
            in_air {
                guard flight_status == InAir
            }
            battery {
                guard battery != Critical
                effect motion -> Free
            }
        }
        success ok {
            effect {
                motion -> Free
            }
        }
        failure ko {
            //...
        }
    }

    skill goto_sol_2 {
        precondition {
            has_authority: authority != Pilot
            in_air       : flight_status == InAir
            not_moving   : motion == Free
            battery_good : battery != Critical
        }
        start {
            motion -> Used
            authority -> Drone
        }
        invariant {
            in_control {
                guard motion == Used
            }
            has_authority {
                guard  authority == Drone
                effect motion -> Free
            }
            in_air {
                guard flight_status == InAir
            }
            battery {
                guard battery != Critical
                effect motion -> Free
            }
        }
        success ok {
            effect {
                motion -> Free
            }
        }
        failure ko {
            //...
        }
    }
}