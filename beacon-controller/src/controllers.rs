use uuid::Uuid;
use crate::controllers::ControllerError::RepositoryError;

use crate::models::actions::BeaconAction;
use crate::models::beacon::{Beacon, BeaconCommand};
use crate::repository;
use crate::repository::{ActionsRepository, BeaconRepository};

pub struct BeaconController {
    repository: BeaconRepository
}

impl BeaconController {
    pub fn new(repository: BeaconRepository) -> Self {
        BeaconController { repository }
    }

    pub fn create_beacon(&self, beacon_command: BeaconCommand) -> ControllerResult<Beacon> {
        self.repository.add_beacon(Beacon {
            id: Uuid::new_v4(),
            state: beacon_command.state,
            ip: beacon_command.ip,
            role: beacon_command.role,
        })
    }

    pub fn get_beacon(&self, id: Uuid) -> ControllerResult<Beacon> {
        self.repository
            .get_beacon(id)
    }

    pub fn get_beacons(&self) -> ControllerResult<Vec<Beacon>> {
        self.repository
            .get_beacons()
    }
}

pub struct ActionsController {
    repository: ActionsRepository
}

impl ActionsController {
    pub fn new(repository: ActionsRepository) -> Self {
        ActionsController { repository }
    }


    pub fn create_action(&self, id: Uuid, beacon_action: BeaconAction) -> ControllerResult<()> {
        self.repository
            .add_action(id, beacon_action)?;
        Ok(())
    }

    pub fn get_undone_action(&self, id: Uuid) -> ControllerResult<Vec<BeaconAction>> {
        self.repository.get_undone_actions(id)
    }

    pub fn mark_as_done(&self, id: Uuid, action_id: Uuid) -> ControllerResult<()> {
        self.repository.mark_as_done(id, action_id)
    }
}


type ControllerResult<T> = anyhow::Result<T>;