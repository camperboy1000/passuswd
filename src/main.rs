use clap::Parser;
use passwd::set_new_password;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// username of the account to act on
    username: Option<String>,

    /// delete the password for the named account (root only); also removes password lock if any
    ///
    /// This is a quick way to delete a password for an account. It will set the named account passwordless. Available to root only.
    ///
    /// Note that if the password was locked, this implicitly removes the password lock as well.
    #[arg(short, long)]
    delete: bool,

    /// lock the password for the named account (root only)
    ///
    /// This option is used to lock the password of specified account and it is available to root only.
    ///
    /// The locking is performed by rendering the encrypted password into an invalid string (by prefixing the encrypted string with an !).
    #[arg(short, long)]
    lock: bool,

    /// unlock the password for the named account (root only)
    ///
    /// This is the reverse of the -l option - it will unlock the account password by removing the ! prefix. This option is available to root only.
    ///
    /// By default passwd will refuse to create a password‐less account (it will not unlock an account that has only "!" as a password). The force option -f will override this protection
    #[arg(short, long)]
    unlock: bool,

    /// force operation
    ///
    /// Force the specified operation.
    #[arg(short, long)]
    force: bool,
}

fn main() {
    let args = Args::parse();

    let username = match users::get_current_username() {
        Some(username) => username.into_string().expect("I'm a little teapot!"),
        None => {
            println!("I couldn't get your username, please tell me whos password to change.");
            return;
        }
    };

    set_new_password(&username);
}
