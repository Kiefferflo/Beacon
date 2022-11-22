use std::process::Command;
use std::{thread, time};

fn main() {
    println!("{}",exec_command("echo Hello world"));
    run();
}

fn read_entry() -> &'static str {
    //placeholder fonction
    return "cmd switch toto"
}

fn transmit_payload_to_next_beacon(entry_string:&str){
    //placeholder fonction
    println!("{}", entry_string)
}

//Read entry messages and transmit them if they are not the mode switch signal
fn read_and_transmit_as_tower(is_operating_as_transmition_tower:bool) -> bool {
    let entry_string: &str = &read_entry(); //get the entry string
    let wake_up_string: &str = "switch";
    if entry_string.contains(wake_up_string) {
        return !is_operating_as_transmition_tower
    } else {
        transmit_payload_to_next_beacon(entry_string);
        return is_operating_as_transmition_tower
    }
}



fn exec_command(cmd: &str) -> String {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", cmd])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("failed to execute process")
    };
    String::from_utf8(output.stdout).unwrap()
}

fn read_and_analyse_entry() -> &'static str {
    //placeholder function
    return "sleep 2"
}

fn new_napping_time(entry_string: &str) -> u64 {
    //placeholder function
    let mut entry_iter = entry_string.split(" ");
    entry_iter.next();
    return entry_iter.next().unwrap().parse::<u64>().unwrap()
}

fn run(){
    //fonction principale d'execution
    let mut is_operating_as_transmition_tower = true; //determine the running mode of the beacon
    let mut cpt: u32 = 0; //compter for the end of the loop during dev because spamming is bad
    let mut time_of_nap:u64 = 0;  //determine the time spent napping every loop
    let mut entry_string:&str ;//string lu en entrée   
    //boucle principale du programme
    loop {
        if is_operating_as_transmition_tower {
            //act as the transmition tower, aka read for change and forward message
            is_operating_as_transmition_tower = read_and_transmit_as_tower(is_operating_as_transmition_tower);
            println!("je suis une tour");
        } else {
            println!("je suis actif");
            entry_string = read_and_analyse_entry();
            time_of_nap = new_napping_time(entry_string);

            thread::sleep(time::Duration::from_secs(time_of_nap));
        }

        if cpt>5 {
            break();
        }
        cpt = cpt+1
    }
}