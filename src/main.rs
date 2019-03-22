extern crate clap;
extern crate dirs;

mod config;

use clap::{App, SubCommand, Arg};
use std::path::Path;
use std::io::prelude::*;
use std::io::{BufReader, BufRead};
use std::fs::{OpenOptions, create_dir_all, read_dir};
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
    println!("{:?}h", duration.as_secs() / 3600);
    Ok(())
}

fn break_tasks() -> Result<(), Box<dyn Error>> {
    let tasks_path = config::base_path();

    for task_entry in read_dir(tasks_path)? {
        let task_path = task_entry?.path();
        if task_path.is_file() {
            let file= BufReader::new(
                OpenOptions::new().read(true).open(&task_path)?
            );

            if file.lines().count() % 2 != 0 {
                track_task(task_path.file_name().and_then(|f| f.to_str()).expect(""))?;
            }
        }
    }
    Ok(())
}

fn continue_tasks() -> Result<(), Box<dyn Error>> {
    let tasks_path = config::base_path();

    let mut most_recent: Option<(String, u64)> = None;

    for task_entry in read_dir(tasks_path)? {
        let task_path = task_entry?.path();
        if task_path.is_file() {
            let file = BufReader::new(
                OpenOptions::new().read(true).open(&task_path)?
            );

            let lines: Vec<String> = file.lines().into_iter().map(|l| l.expect("")).collect();

            if lines.len() % 2 == 0 && lines.len() > 0 {
                let last_timestamp = lines.last().expect("").parse::<u64>()?;
                let (_, current_most_recent_timestamp) = most_recent.clone().unwrap_or((String::new(), 0u64));

                if last_timestamp > current_most_recent_timestamp {
                    most_recent = Some((task_path.file_name().and_then(|f| f.to_str()).expect("").to_string(), last_timestamp))
                }
            }
        }
    }

    most_recent.and_then(|(most_recent_task, _)| {
       track_task(most_recent_task.as_str()).err()
    });

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

    let break_command_name = "break";
    let break_command = SubCommand::with_name(break_command_name);

    let continue_command_name = "continue";
    let continue_command = SubCommand::with_name(continue_command_name);

    let matches = App::new("track")
        .subcommand(time_sub_command)
        .subcommand(task_time_command)
        .subcommand(break_command)
        .subcommand(continue_command)
        .get_matches();

    if let Some(command) = matches.subcommand_matches(time_command_name) {
        let task = command.value_of("").expect("");
        track_task(task)?;
    }

    if let Some(command) = matches.subcommand_matches(task_time_command_name) {
        let task = command.value_of("").expect("");
        task_time(task)?;
    }

    if let Some(_command) = matches.subcommand_matches(break_command_name) {
        break_tasks()?;
    }

    if let Some(_command) = matches.subcommand_matches(continue_command_name) {
        continue_tasks()?;
    }

    Ok(())
}
