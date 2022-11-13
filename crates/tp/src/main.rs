mod cli;
mod config;
mod error;
mod tmux;

use crate::tmux::create_session;
use anyhow::Result;
use config::PathOptions;
use skim::prelude::*;
use std::collections::VecDeque;
use std::io::Cursor;
use std::path::PathBuf;
use std::string::ToString;
use std::vec::Vec;

fn search_projects() -> String {
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .prompt(Some(" "))
        .multi(false)
        .bind(vec!["esc:abort"])
        .build()
        .unwrap();

    let config = config::get();

    let input = find_projects(config.paths).join("\n");

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));

    Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(Vec::new)
        .first()
        .expect("should not crash")
        .output()
        .to_string()
}

fn find_projects(paths: Vec<PathOptions>) -> Vec<String> {
    let mut projects = Vec::new();

    for path_opts in paths {
        if path_opts.git {
            projects.extend(find_repos(PathBuf::from(path_opts.path), path_opts.exlude));
        } else {
            projects.extend(find_directories(
                PathBuf::from(path_opts.path),
                path_opts.exlude,
            ));
        }
    }

    projects
}

fn find_directories(path: PathBuf, exlude: Option<Vec<String>>) -> Vec<String> {
    let mut dirs = Vec::new();
    let mut to_search = VecDeque::new();

    to_search.extend(
        std::fs::read_dir(path)
            .unwrap()
            .map(|dir_entry| dir_entry.expect("Found non-valid utf8 path").path()),
    );

    while let Some(file) = to_search.pop_front() {
        let file_path = file.clone().into_os_string().into_string().unwrap();

        let is_excluded = match &exlude {
            Some(exlude) => exlude.iter().any(|exl| file_path.contains(exl)),
            None => false,
        };

        if !is_excluded {
            dirs.push(file_path);
        }
    }

    dirs
}

fn find_repos(path: PathBuf, exlude: Option<Vec<String>>) -> Vec<String> {
    let mut repos = Vec::new();
    let mut to_search = VecDeque::new();

    to_search.push_front(path);

    while let Some(file) = to_search.pop_front() {
        let file_path = file.clone().into_os_string().into_string().unwrap();

        let is_excluded = match &exlude {
            Some(exlude) => exlude.iter().any(|exl| file_path.contains(exl)),
            None => false,
        };

        if !is_excluded {
            if git2::Repository::open(&file).is_ok() {
                repos.push(file_path);
            } else if file.is_dir() {
                to_search.extend(
                    std::fs::read_dir(file)
                        .unwrap()
                        .map(|dir_entry| dir_entry.expect("Found non-valid utf8 path").path()),
                );
            }
        }
    }

    repos
}

fn main() -> Result<()> {
    let cli = cli::create_app();

    let matches = cli.get_matches();

    let selected_project = PathBuf::from(search_projects());

    create_session(
        selected_project.to_str().unwrap(),
        selected_project.file_name().unwrap().to_str().unwrap(),
    )?;

    Ok(())
}
