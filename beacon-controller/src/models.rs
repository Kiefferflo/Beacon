pub mod beacon {
    use std::net::IpAddr;
    use uuid::Uuid;

    #[derive(Clone, Copy)]
    pub struct Beacon {
        pub id: Uuid,
        pub state: BeaconState,
        pub ip: IpAddr,
        pub role: BeaconRole
    }

    #[derive(Clone, Copy)]
    pub enum BeaconRole {
        Proxy,
        Agent
    }

    #[derive(Clone, Copy)]
    pub struct BeaconCommand {
        pub state: BeaconState,
        pub ip: IpAddr,
        pub role: BeaconRole
    }

    #[derive(Clone, Copy)]
    pub enum BeaconState {
        Running,
        Sleeping,
        Stopped
    }
}

pub mod actions {
    use std::process::Command;
    use std::time::Duration;
    use crate::models::beacon::BeaconRole;

    #[derive(Clone)]
    pub enum BeaconAction {
        ExecuteCommand(Command),
        LoadExecutable { executable: Vec<u8> },
        FallAsleep(Duration),
        ApplyRole(BeaconRole),
    }
}