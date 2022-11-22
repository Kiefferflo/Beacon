use std::process::Command;

fn main() {
    println!("{}",exec_command("echo Hello world"));
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