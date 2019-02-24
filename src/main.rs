extern crate clap;
extern crate dirs;

mod config;

use clap::{App, SubCommand, Arg};
use std::path::Path;
use std::io::prelude::*;
use std::io::{BufReader, BufRead};
use std::fs::{OpenOptions, create_dir_all};
use std::error::Error;
use std::time::{SystemTime, Duration};

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

fn task_time(task_name: &str) -> Result<(), Box<dyn Error>> {
    let task_path = config::base_path().join(Path::new(task_name));

    let file = BufReader::new(OpenOptions::new()
        .read(true)
        .open(task_path)?);

    let mut duration = Duration::new(0, 0);
    let mut previous_time: Option<u64> = Option::None;
    for line in file.lines() {
        if previous_time.is_some() {
            let next = line?.as_str().parse::<u64>()?;
            duration = duration + Duration::new(next - previous_time.expect(""), 0);
            previous_time = None;
        } else {
            previous_time = Some(line?.as_str().parse::<u64>()?);
        }
    }
    println!("{:?}", duration);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let time_command_name = "time";
    let time_sub_command = SubCommand::with_name(time_command_name).arg(
        Arg::with_name("").takes_value(true).required(true)
    );

    let task_time_command_name = "sum";
    let task_time_command = SubCommand::with_name(task_time_command_name).arg(
        Arg::with_name("").takes_value(true).required(true)
    );

    let matches = App::new("track")
        .subcommand(time_sub_command)
        .subcommand(task_time_command)
        .get_matches();

    if let Some(command) = matches.subcommand_matches(time_command_name) {
        let task = command.value_of("").expect("");
        track_task(task)?;
    }

    if let Some(command) = matches.subcommand_matches(task_time_command_name) {
        let task = command.value_of("").expect("");
        task_time(task)?;
    }


    Ok(())
}
