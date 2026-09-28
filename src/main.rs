use std::io::{self, Write, stdin, stdout};
mod ops_std;

fn main() {
    println!("ops v0.0.0");
    let un = whoami::username().expect("err");
    let mut path: &mut String = &mut String::from("C:\\Users\\".to_owned() + un.as_str());
    while true {
        print!("{}@{}>", un, path);
        stdout().flush().expect("failed to flush stdout");
        let buf: &mut String = &mut String::new();
        stdin().read_line(buf).expect("failed to read line");
        let mut trim= buf.trim().to_string();
        ops_std::link_cmd(trim.as_str(), un.as_str(), &mut path);
    }   
}
