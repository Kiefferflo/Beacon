use std::sync::Arc;
use uuid::Uuid;
use warp::Filter;
use crate::controllers::{ActionsController, BeaconController};
use crate::repository::{ActionsRepository, BeaconRepository};

mod controllers;
pub mod models;
mod repository;

pub async fn run() {
    let beacon_controller = Arc::new(BeaconController::new(BeaconRepository));
    let actions_controller = Arc::new(ActionsController::new(ActionsRepository));
    let post_beacon = warp::path("beacons")
        .and(warp::post())
        .and(warp::body::json())
        .map(beacon_controller.clone().create_beacon);
    let get_beacons = warp::path!("beacons").map(beacon_controller.clone().get_beacons);
    let get_beacon = warp::path!("beacons" / Uuid).map(|id|beacon_controller.clone().get_beacon(id));
    let get_actions = warp::path!("beacons" / Uuid / "actions").map(|id|actions_controller.clone().get_undone_action(id));
    let post_actions = warp::path!("beacons" / Uuid / "actions")
        .and(warp::post())
        .and(warp::body::json())
        .map(|id, body|actions_controller.clone().create_action(id, body));
    let put_action = warp::path!("beacons" / Uuid / "actions" / Uuid / "status" / "done")
        .and(warp::put())
        .map(|bid, aid|actions_controller.clone().mark_as_done(bid, aid));

    let routes =
        post_beacon
            .or(get_beacons)
            .or(get_beacon)
            .or(get_actions)
            .or(post_actions)
            .or(put_action);

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await?
}