use std::env::args;
use std::fs;

use hyprland::dispatch::Dispatch;
use regex::Regex;
use hyprland::data::{Monitors, Monitor};
use hyprland::shared::{HyprError, HyprData, HyprDataVec};


fn main() -> Result<(),HyprError> {


    let raw_argument: Vec<_> = args().into_iter().collect();
    let raw_argument: Option<String> = raw_argument.get(1).cloned();

    if raw_argument.is_none() {
        println!("Argunment is not added please added");
        return Ok(());
    }
    let argument = match raw_argument.unwrap().parse() {
        Ok(x) => x,
        Err(_) => {
            println!("Argunment is not a number");
            0
        }
    };

    if argument == 0 {
        return Ok(());
    }

    // let mons_name: Vec<String> = all_mons_name();
    let workspaces_config = open_config_hyprland();
    let focused_monitor = focus_monitor().unwrap().unwrap();

    let mut workspaces_focused_monitor: Vec<&String> = Vec::new();

    for i in 0..workspaces_config.len() {
        if focused_monitor.name == workspaces_config.get(i).unwrap().0 {
            workspaces_focused_monitor.push(&workspaces_config.get(i).unwrap().1)
        }
    }
    
    if argument > workspaces_focused_monitor.get(workspaces_focused_monitor.len() - 1).unwrap().parse().unwrap() {
        println!("Argunment is greater than the workspaces");
        return Ok(());
    }
    
    match move_to_workspace(workspaces_focused_monitor.get(argument - 1).unwrap()){
        Ok(_) =>{},
        Err(e) => {
            println!("{}",e);
            return Err(e)
        }
    }

    Ok(())
}

fn focus_monitor() -> Result<Option<Monitor>,HyprError> {
    let monitors = Monitors::get()?.to_vec();
    for mon in monitors {
        if mon.focused {
             return Ok(Some(mon));
        }
    }
    Ok(None)
}

fn open_config_hyprland() -> Vec<(String,String)> {
    let content = fs::read_to_string("/home/robert/.config/hypr/hyprland.conf").unwrap();
    let workspace_search = Regex::new(r"workspace\s*=\s*([^,]+),\s*(\d+)").unwrap();

    let results: Vec<(String, String)> = workspace_search
        .captures_iter(&content)
        .map(|capture| {
            let valor_string = capture.get(1).unwrap().as_str().to_string();
            let numero = capture.get(2).unwrap().as_str().to_string();
            (valor_string, numero)
        })
        .collect();
    results
}

fn move_to_workspace(workspace: &str) -> Result<(),HyprError> {
    Dispatch::call(
        hyprland::dispatch::DispatchType::Workspace(
            hyprland::dispatch::WorkspaceIdentifierWithSpecial::Name(workspace))
        )
}
