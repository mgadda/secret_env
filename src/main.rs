use std::env;
use std::io::{stdin,stdout,Write};
use std::os::unix::process::CommandExt;
use std::process::{Command, exit};

fn main() {
    // get the arguments
    let args: Vec<String> = env::args().collect();

    // ensure we have the correct number of arguments
    if args.len() < 3 {
        eprintln!("Usage: secret_env <ENV_VAR_NAME> <COMMAND> [ARGS...]");
        exit(1);
    }

    // get the environment variable name and command to execute
    let env_var_name = &args[1];
    let command = &args[2];
    let command_args = &args[3..];

    // prompt for the secret
    print!("{}: ", env_var_name);
    stdout().flush().unwrap();
    let secret = rpassword::read_password().unwrap();


    // Exec as command. Good bye!
    Command::new(command)
      .env(env_var_name, secret)
      .args(command_args)
      .exec();
}
