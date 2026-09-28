use std::{fs, process::Command};
fn create_full_path(path: &str, add_on: &str) -> String {
    let r = format!("{}{}{}", path, "\\", add_on);
    String::from(r.trim())
}

pub fn link_cmd(cmd: &str, username: &str, path: &mut String) {
    let fragments: Vec<&str> = cmd.split(" ").collect();
    let o_cmd = fragments[0].trim();
    match o_cmd {
        "touch" => std_touch(path, cmd).expect("touch: failed to touch"),
        "cd" => std_cd(path, cmd),
        "ls" => std_ls(path, cmd),
        "cat" => std_cat(path, cmd),
        "ckmm" => std_ckmm(cmd),
        "win" => std_win(cmd),
        "open" => std_open(cmd),
        &_ => println!("{} is not a valid command", o_cmd),
    }
}

pub fn std_touch(orig: &str, path: &str) -> std::io::Result<()> {
    let fragments: Vec<&str> = path.split(" ").collect();

    if (fragments.len() < 2) {
        println!("usage:\n\ttouch [filename]\ncreate file with filename as its name");
        return Ok(());
    }

    println!("{}", create_full_path(orig, fragments[1]));
    fs::write(create_full_path(orig, fragments[1]), "").expect("error creating file");
    Ok(())
}

pub fn std_cd(path: &mut String, cmd: &str) {
    let fragments: Vec<&str> = cmd.split(" ").collect();

    if (fragments[1] == "-f") {
        if let Err(e) = fs::read_dir(fragments[2]) {
            eprintln!("cd: {} is not a valid directory", fragments[2]);
            return;
        }
        *path = String::from(fragments[2]);
        return;
    } else if (fragments[1] == "..") {
        let mut splits: Vec<&str> = path.split("\\").collect();
        if splits.len() > 1 {
            splits.pop();
        }
        *path = String::from(splits.join("\\"));
        return;
    }
    let mut r = path.clone();
    r.push_str("\\");
    r.push_str(fragments[1]);

    if let Err(e) = fs::read_dir(&r) {
        eprintln!("cd: {} is not a valid directory", r);
        return;
    }

    path.push_str("\\");
    path.push_str(fragments[1]);
}

pub fn std_ls(path: &mut String, cmd: &str) {
    for entry in fs::read_dir(path).unwrap() {
        let e = entry.unwrap();
        let dt = e.file_type().unwrap();

        let mut specific_type = String::new();
        if dt.is_dir() {
            specific_type = "dir".to_owned();
        }
        if dt.is_file() {
            specific_type = "file".to_owned();
        }
        println!("{} {}", e.file_name().into_string().unwrap(), specific_type);
    }
}

pub fn std_cat(path: &mut String, cmd: &str) {
    let fragments: Vec<&str> = cmd.split(" ").collect();
    for frag in fragments {
        if frag != "cat" {
            if let Err(e) = fs::read(create_full_path(path, frag)) {
                eprintln!("failed to read {}", create_full_path(path, frag));
                continue;
            }
            let txt = fs::read(create_full_path(path, frag));
            println!("{}", String::from_utf8(txt.unwrap()).unwrap());
        }
    }
} 

pub fn std_ckmm(cmd: &str) {
    let fragments: Vec<&str> = cmd.split(" ").collect();
    let first_addr: u32 = u32::from_str_radix(fragments[1].strip_prefix("0x").unwrap(), 16).unwrap();
    let second_addr: u32 = u32::from_str_radix(fragments[2].strip_prefix("0x").unwrap(), 16).unwrap();

    println!("on {} to {}", first_addr, second_addr);
    unsafe {
        for i in first_addr..second_addr {
            let addr: *const u8 = i as *const u8;
            println!("[{}] = {}", i, *addr);
        }
    }
}

pub fn std_win(cmd: &str) {
    let fragments: Vec<&str> = cmd.split(" ").collect();
    let c = Command::new(fragments[1]).args(fragments[2..].iter()).output();
    if c.is_err() {
        eprintln!("win: failed to run {}", fragments[1..].join(" "));
        return;
    }
    
    let good = c.expect("err");
    if good.status.success() {
        let std_out = String::from_utf8_lossy(&good.stdout);
        println!("{}", std_out);
    } else {
        eprintln!("win: failed to run {}", fragments[1..].join(" "));
    }
}

pub fn std_open(cmd: &str) {
    let fragments: Vec<&str> = cmd.split(" ").collect();
    let r = 
        Command::new(fragments[1])
        .args(fragments[2..].iter())
        .spawn();

    if (r.is_err()) {
        eprintln!("open: failed to open {}", fragments[1]);
    }
}