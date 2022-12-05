use std::collections::{BTreeMap, VecDeque};
use uuid::Uuid;
use crate::models::actions::BeaconAction;

use crate::models::beacon::Beacon;

pub struct KVStore {
    beacons: BTreeMap<Uuid, Beacon>,
    actions: BTreeMap<Uuid, VecDeque<BeaconAction>>
}

impl KVStore {
    pub fn new() -> Self {
        KVStore { beacons: BTreeMap::new(), actions: BTreeMap::new() }
    }

    pub fn get_beacon(&self, key: Uuid) -> Option<Beacon> {
        self.beacons.get(&key).cloned()
    }

    pub fn list_beacon(&self) -> Vec<Beacon> {
        self.beacons.values()
            .cloned()
            .collect()
    }

    pub fn insert_beacon(&mut self, beacon: Beacon) -> Option<Beacon> {
        self.beacons.insert(beacon.id, beacon)
    }

    pub fn get_actions(&self, key: Uuid) -> Option<Vec<BeaconAction>> {
        self.actions.get(&key)?
            .into_iter()
            .cloned()
            .collect()
    }

    pub fn dequeue_action(&mut self, key: Uuid) -> Option<BeaconAction> {
        self.actions.get(&key)?.pop_back()
    }

    pub fn enqueue_action(&mut self, key: Uuid, action: BeaconAction) -> Option<()> {
        self.actions.get(&key)?.push_front(action);
        Some(())
    }
}