use std::process::Command;

fn main() {
    println!("{}",exec_command("echo Hello world"));
    run();
}

fn read_entry() -> &'static str {
    //placeholder fonction
    return "cmd swittch toto"
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


fn run(){
    //fonction principale d'execution
    let mut is_operating_as_transmition_tower = true; //determined the running mode of the beacon
    let mut cpt: u32 = 0;
    //boucle principale du programme
    loop {
        if is_operating_as_transmition_tower {
            //act as the transmition tower, aka read for change and forward message
            is_operating_as_transmition_tower = read_and_transmit_as_tower(is_operating_as_transmition_tower);
            println!("je suis une tour");
        } else {
            println!("je suis actif");
        }

        if cpt>5 {
            break();
        }
        cpt = cpt+1
    }
}