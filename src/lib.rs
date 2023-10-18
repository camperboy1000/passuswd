use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Seek},
};

static FILE_PATH: &str = "shadow";

pub fn set_new_password(username: &String) {
    println!("Changing password for user {}.", username);
    let prompt = String::from("Current Password: ");

    let password = match rpassword::prompt_password(&prompt) {
        Ok(password) => password,
        Err(_) => return,
    };

    let file = match File::options().read(true).write(true).open(FILE_PATH) {
        Ok(file) => file,
        Err(_) => {
            println!("I couldn't check your password...");
            println!("Do I have the proper permissions?");
            return;
        }
    };

    let mut file_reader = BufReader::new(&file);
    let file_writer = BufWriter::new(&file);

    // let shadow_lines: Vec<String> = file_reader
    // .lines()
    // .map(|line| line.unwrap_or_default())
    // .collect();

    let mut seek_position = None;
    let mut line = String::new();
    let mut line_split: Vec<&str>;

    while let Ok(_) = file_reader.read_line(&mut line) {
        line_split = line.split(':').collect();

        if line_split[0] == username {
            seek_position = Some(file_reader.stream_position().unwrap_or_default());
            break;
        }

        line.clear();
    }

    // for index in 0..shadow_lines.len() {
    //     line = shadow_lines[index].split(':').collect();
    //
    //     if line[0] == username {
    //         seek_position = file_reader.stream_position().unwrap_or_default();
    //         break;
    //     }
    // }

    // file_writer.seek(SeekFrom::into);

    println!("Position {}:\n{}", seek_position.unwrap(), line);
}
