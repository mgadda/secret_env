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


    Command::new(command)
      .env(env_var_name, secret)
      .args(command_args)
      .exec();


    // set the environment variable
    // env::set_var(env_var_name, secret);

    // Attempt #1 - requires path
    // execute the command with the environment variable set
    // let status = Command::new(command)
    //   .args(command_args)
    //   .status()
    //   .expect("failed to execute command");

    // Attempt #2 - requires /usr/bin/env to exist
    // let status = Command::new("/usr/bin/env")
    //   .arg(command)
    //   .args(command_args)
    //   .status()
    //   .unwrap_or_else(|e| {
    //       eprintln!("failed to execute command: {}", e);
    //       exit(1);
    //   });


    // exit with the same status code as the executed command
    //exit(status.code().unwrap_or(1));
}
