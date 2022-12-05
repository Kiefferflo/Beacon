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
        }).map_err(|e| RepositoryError(e))
    }

    pub fn get_beacon(&self, id: Uuid) -> ControllerResult<Beacon> {
        self.repository
            .get_beacon(id)
            .map_err(|e| RepositoryError(e))
    }

    pub fn get_beacons(&self) -> ControllerResult<Vec<Beacon>> {
        self.repository
            .get_beacons()
            .map_err(|e| RepositoryError(e))
    }
}

pub struct ActionsController {
    repository: ActionsRepository
}

impl ActionsController {
    pub fn create_action(&mut self, id: Uuid, beacon_action: BeaconAction) -> ControllerResult<()> {
        self.repository
            .enqueue_action(id, beacon_action)
            .map_err(|e| RepositoryError(e))
    }

    pub fn get_latest_action(&mut self, id: Uuid) -> ControllerResult<BeaconAction> {
        self.repository.dequeue_action(id)
            .map_err(|e| RepositoryError(e))
    }
}

pub enum ControllerError {
    RepositoryError(repository::RepositoryError)
}

type ControllerResult<T> = Result<T, ControllerError>;