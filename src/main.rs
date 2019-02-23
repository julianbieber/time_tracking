extern crate clap;
extern crate dirs;

mod config;

use clap::{App, SubCommand, Arg};
use std::path::Path;
use std::io::prelude::*;
use std::fs::{OpenOptions, create_dir_all};
use std::error::Error;
use std::time::SystemTime;

fn track_task(task_name: &str) -> Result<(), Box<dyn Error>> {
    create_dir_all(config::base_path())?;
    let task_path = config::base_path().join(Path::new(task_name));

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .write(true)
        .open(task_path)?;
    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    let timestamp = time.as_secs();
    writeln!(file, "{}", timestamp)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let time_command_name = "time";
    let time_sub_command = SubCommand::with_name(time_command_name).arg(
        Arg::with_name("").takes_value(true).required(true)
    );

    let matches = App::new("track")
        .subcommand(time_sub_command)
        .get_matches();

    if let Some(time_command) = matches.subcommand_matches(time_command_name) {
        let task = time_command.value_of("").expect("");
        track_task(task)?;
    }


    Ok(())
}
