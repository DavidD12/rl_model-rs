type Battery
type GeoPoint
type Float

skillset uav {
    data {
        battery: Battery
        position: GeoPoint period 1.0
        home: GeoPoint    
    }

    resource {
        authority {
            state { Free Pilot Software }
            initial Pilot
            transition {
                Free     -> Software
                Free     -> Pilot
                Software -> Free
                Software -> Pilot
                Pilot    -> Free            
            }
        }
        flight_status {
            state { NotReady OnGround InAir }
            initial NotReady
            transition all
        }
        motion {
            state { Available Used }
            initial Available
            transition all
        }
        battery {
            state { Good Low Critical }
            initial Good
            transition {
                Good -> Low
                Good -> Critical
                Low  -> Critical
            }
        }
    }

    event take_authority {
        guard  authority != Pilot
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
            has_authority: authority == Software
            on_ground    : flight_status == OnGround
            motion_avail : motion == Available
            battery_good : battery == Good
        }
        start motion -> Used
        invariant {
            in_control {
                guard motion == Used
            }
            has_authority {
                guard  authority == Software
                effect motion -> Available
            }
            battery {
                guard battery != Critical
                effect motion -> Available
            }
        }
        progress {
            period  1.0
            output height: Float
        }
        interrupt {
            interrupting true
            effect motion -> Available
        }
        success at_altitude {
            effect motion -> Available
            postcondition flight_status == InAir
        }
        failure {
            grounded {
                effect motion -> Available
                postcondition flight_status == OnGround
            }
            emergency {
                effect motion -> Available
                postcondition flight_status == InAir
            }
        }
    }

    skill goto {
        precondition {
            has_authority: authority != Pilot
            in_air       : flight_status == InAir
            moving_avail : motion == Available
            battery_good : battery != Critical
        }
        start motion -> Used
        invariant {
            in_control {
                guard motion == Used
            }
            has_authority {
                guard  authority == Software
                effect motion -> Available
            }
            in_air {
                guard flight_status == InAir
            }
            battery {
                guard battery != Critical
                effect motion -> Available
            }
        }
        success ok {
            effect {
                motion -> Available
            }
        }
        failure ko {
            //...
        }
    }

    skill goto_sol_1 {
        precondition {
            has_authority: authority == Software
            in_air       : flight_status == InAir
            not_moving   : motion == Available
            battery_good : battery != Critical
        }
        start motion -> Used
        invariant {
            in_control {
                guard motion == Used
            }
            has_authority {
                guard  authority == Software
                effect motion -> Available
            }
            in_air {
                guard flight_status == InAir
            }
            battery {
                guard battery != Critical
                effect motion -> Available
            }
        }
        success ok {
            effect {
                motion -> Available
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
            not_moving   : motion == Available
            battery_good : battery != Critical
        }
        start {
            motion -> Used
            authority -> Software
        }
        invariant {
            in_control {
                guard motion == Used
            }
            has_authority {
                guard  authority == Software
                effect motion -> Available
            }
            in_air {
                guard flight_status == InAir
            }
            battery {
                guard battery != Critical
                effect motion -> Available
            }
        }
        success ok {
            effect {
                motion -> Available
            }
        }
        failure ko {
            //...
        }
    }
}