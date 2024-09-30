use mysql::Pool;
use colored::Colorize;

use crate::database::plant_controller;

pub fn plant_cmd(args: &Vec<String>, pool: Pool) {
    let cmd = if args.len() > 0 {args[0].clone()} else {format!("no command entered")};
    match cmd.as_str() {
        "view" => {
            if args.len() == 2 {
                let plant_id = args[1].parse().unwrap_or(0);
                view_plant(pool, plant_id);
            }
            else {
                eprintln!("{}: {}", "Error".red(), "plant view requires a plant id")
            }
        },
        "help" => help(),
        _ => eprintln!("{} command '{cmd}' not found for plant. Run 'container help' to see availavle commands", "Error:".red())
    }
}

fn view_plant(pool: Pool, plant_id: u64) {
    if plant_id == 0 {
        eprintln!("{}: {}", "Error".red(), format!("'{plant_id}', is not a valid plant id"));
        return;
    }

    match plant_controller::view_plant(pool, plant_id) {
        Ok(plants) => {},
        Err(e) => eprintln!("{}:, {:?}", "Err".red(), e)
    }
}

fn help() {
    let help = 
    r#"container help: 
    useage:
        plant [command] [options]

    commands:
        view <plant_id>"#;

    println!("{help}");
}