use std::collections::VecDeque;
use std::sync::{Arc, LockResult, Mutex, MutexGuard, PoisonError};

use uuid::Uuid;

use crate::kv::KVStore;
use crate::models::actions::BeaconAction;
use crate::models::beacon::Beacon;
use crate::repository::RepositoryError::{BeaconNotFound, InsertionError, LockError};

pub struct BeaconRepository { kv: Arc<Mutex<KVStore>> }

type RepositoryResult<T> = Result<T, RepositoryError>;

impl BeaconRepository {
    pub fn get_beacon(&self, beacon_id: Uuid) -> RepositoryResult<Beacon> {
        self.kv.lock()
            .map_err(|_| LockError)?
            .get_beacon(beacon_id)
            .ok_or_else(|| BeaconNotFound(beacon_id))
    }

    pub fn get_beacons(&self) -> RepositoryResult<Vec<Beacon>> {
        let beacons = self.kv.lock()
            .map_err(|_| LockError)?
            .list_beacon();
        Ok(beacons)
    }

    pub fn add_beacon(&self, beacon: Beacon) -> RepositoryResult<Beacon> {
        let beacon = self.kv.lock()
            .map_err(|_| LockError)?
            .insert_beacon(beacon)
            .ok_or_else(|| InsertionError(beacon))?;
        Ok(beacon)
    }
}

pub struct ActionsRepository { kv: Arc<Mutex<KVStore>> }

impl ActionsRepository {
    pub fn enqueue_action(&mut self, beacon_id: Uuid, action: BeaconAction) -> RepositoryResult<()> {
        self.kv.get_mut()
            .map_err(|_| LockError)?
            .enqueue_action(beacon_id, action)
            .ok_or_else(|| BeaconNotFound(beacon_id))
    }

    pub fn dequeue_action(&mut self, beacon_id: Uuid) -> RepositoryResult<BeaconAction> {
        self.kv.get_mut()
            .map_err(|_| LockError)?
            .dequeue_action(beacon_id)
            .ok_or_else(|| BeaconNotFound(beacon_id))
    }

    pub fn list_actions(&self, beacon_id: Uuid) -> RepositoryResult<Vec<BeaconAction>> {
        self.kv.lock()
            .map_err(|_| LockError)?
            .get_actions(beacon_id)
            .ok_or_else(|| BeaconNotFound(beacon_id))
    }
}

pub enum RepositoryError {
    LockError,
    BeaconNotFound(Uuid),
    InsertionError(Beacon)
}