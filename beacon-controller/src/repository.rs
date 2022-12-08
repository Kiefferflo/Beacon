use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::sync::{Arc, LockResult, Mutex, MutexGuard, PoisonError};
use uuid::Uuid;

use crate::kv::KVStore;
use crate::models::actions::{BeaconAction, BeaconCommand, BeaconCommandStatus};
use crate::models::beacon::Beacon;
use crate::repository::RepositoryError::{BeaconNotFound, InsertionError, LockError};

pub struct BeaconRepository;

type RepositoryResult<T> = anyhow::Result<T>;

impl BeaconRepository {
    pub fn get_beacon(&self, beacon_id: Uuid) -> RepositoryResult<Beacon> {
        let buf = std::fs::read_to_string("beacons.json")?;
        let result: Vec<Beacon> = serde_json::from_str(&buf)?;
        let beacon = result.into_iter()
            .find(|b| b.id == beacon_id)?;
        Ok(beacon)
    }

    pub fn get_beacons(&self) -> RepositoryResult<Vec<Beacon>> {
        let buf = std::fs::read_to_string("beacons.json")?;
        let beacons: Vec<Beacon> = serde_json::from_str(&buf)?;
        Ok(beacons)
    }

    pub fn add_beacon(&self, beacon: Beacon) -> RepositoryResult<Beacon> {
        let mut beacons = self.get_beacons()?;
        beacons.push(beacon.clone());
        let json = serde_json::to_string(&beacons)?;
        std::fs::write("beacons.json", json)?;
        Ok(beacon)
    }
}

pub struct ActionsRepository;

impl ActionsRepository {
    pub fn get_undone_actions(&self, beacon_id: Uuid) -> RepositoryResult<Vec<BeaconAction>> {
        let buf = std::fs::read_to_string(format!("actions_{beacon_id}.json"))?;
        let actions: Vec<BeaconAction> = serde_json::from_str(&buf)?;
        let actions = actions.into_iter()
            .filter(|a| a.status == BeaconCommandStatus::Undone)
            .collect();
        Ok(actions)
    }

    pub fn add_action(&self, beacon_id: Uuid, action: BeaconAction) -> RepositoryResult<BeaconAction> {
        let buf = std::fs::read_to_string(format!("actions_{beacon_id}.json"))?;
        let mut actions: Vec<BeaconAction> = serde_json::from_str(&buf)?;
        actions.push(action);
        let json = serde_json::to_string(&actions)?;
        std::fs::write(format!("actions_{beacon_id}"), json)?;
        Ok(action)
    }

    pub fn mark_as_done(&self, beacon_id: Uuid, action_id: Uuid) -> RepositoryResult<()> {
        let buf = std::fs::read_to_string(format!("actions_{beacon_id}.json"))?;
        let mut actions: Vec<BeaconAction> = serde_json::from_str(&buf)?;
        for (i, action) in actions.into_iter().enumerate() {
            if action.id == action_id {
                actions[i].status = BeaconCommandStatus::Done
            }
        }
        let json = serde_json::to_string(&actions)?;
        std::fs::write(format!("actions_{beacon_id}"), json)?;
        Ok(())
    }
}
