pub mod beacon {
    use std::net::IpAddr;
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Clone, Copy, Deserialize, Serialize)]
    pub struct Beacon {
        pub id: Uuid,
        pub state: BeaconState,
        pub ip: IpAddr,
        pub role: BeaconRole
    }

    #[derive(Clone, Copy, Deserialize, Serialize)]
    pub enum BeaconRole {
        Proxy,
        Agent
    }

    #[derive(Clone, Copy, Deserialize, Serialize)]
    pub struct BeaconCommand {
        pub state: BeaconState,
        pub ip: IpAddr,
        pub role: BeaconRole
    }

    #[derive(Clone, Copy, Deserialize, Serialize)]
    pub enum BeaconState {
        Running,
        Sleeping,
        Stopped
    }
}

pub mod actions {
    use std::process::Command;
    use std::time::Duration;
    use uuid::Uuid;
    use crate::models::beacon::{Beacon, BeaconRole};

    #[derive(Clone, Copy, Deserialize, Serialize)]
    pub struct BeaconAction {
        pub id: Uuid,
        pub data: BeaconCommand,
        pub status: BeaconCommandStatus
    }

    #[derive(Clone, Copy, Deserialize, Serialize)]
    pub enum BeaconCommandStatus {
        Undone,
        Done
    }

    #[derive(Copy, Deserialize, Serialize)]
    pub enum BeaconCommand {
        ExecuteCommand(&'static Command),
        LoadExecutable { executable: Vec<u8> },
        FallAsleep(Duration),
        StartProxy,
        StopProxy,
    }

    impl Clone for BeaconCommand {
        fn clone(&self) -> Self {
            match self {
                BeaconCommand::ExecuteCommand(command) => BeaconCommand::ExecuteCommand(
                    Command::new(command.get_program())
                        .args(command.get_args())
                        .envs(command.get_envs())
                        .current_dir(command.get_current_dir())
                ),
                BeaconCommand::LoadExecutable { executable } => BeaconCommand::LoadExecutable { executable: executable.clone() },
                BeaconCommand::FallAsleep(duration) => BeaconCommand::FallAsleep(duration.clone()),
                BeaconCommand::StartProxy => BeaconCommand::StartProxy,
                BeaconCommand::StopProxy => BeaconCommand::StopProxy
            }
        }
    }
}