use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn set_new_password(username: &String) {
    println!("Changing password for user {}.", username);
    let prompt = String::from("Current Password: ");

    let password = match rpassword::prompt_password(&prompt) {
        Ok(password) => password,
        Err(_) => return,
    };

    let file = match File::options().read(true).write(true).open("/etc/shadow") {
        Ok(file) => file,
        Err(_) => {
            println!("I couldn't check your password...");
            println!("Do I have the proper permissions?");
            return;
        }
    };

    let mut line_index = Default::default();
    let file_reader = BufReader::new(file);
    let shadow_lines: Vec<String> = file_reader
        .lines()
        .map(|line| line.unwrap_or_default())
        .collect();

    for index in 0..shadow_lines.len() {
        let line: Vec<&str> = shadow_lines[index].split(':').collect();

        if line[0] == username {
            line_index = index;
            break;
        }
    }

    println!("{}", shadow_lines[line_index]);
}
