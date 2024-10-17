use mysql::serde_json::Error;
use mysql::Pool;
use colored::Colorize;

use crate::database::plant_controller::{self, Plant};
use crate::cli::{prompt_input, prompt_input_f64};

use super::{container, garden};

pub fn plant_cmd(args: &Vec<String>, pool: Pool) {
    let cmd = if args.len() > 0 {args[0].clone()} else {format!("no command entered")};
    match cmd.as_str() {
        "add" => add_plant(pool, &args[1..].to_vec()),
        "view" => {
            if args.len() == 2 {
                let plant_id = args[1].parse().unwrap_or(0);
                view_plant(pool, plant_id);
            }
            else {
                eprintln!("{}: {}", "Error".red(), "plant view requires a plant id")
            }
        },
        "container" => {view_plants(pool, &args[1..].to_vec());},
        "help" => help(),
        _ => eprintln!("{} command '{cmd}' not found for plant. Run 'plant help' to see availavle commands", "Error:".red())
    }
}

fn add_plant(pool: Pool, args: &Vec<String>) {
    println!("{}", "Enter plant info".green());

    let mut container_name = None;
    let mut garden_name = None;
    let mut location = None;
    let mut species = None;
    let mut variety = None;

    for arg in args.iter() {
        let option: Vec<&str> = arg.split('=').collect();

        if option.len() == 2{
            let (key, val) = (option[0], option[1]);

            match key {
                "container" => container_name = Some(val.to_string().chars().filter(|c| *c != '\"').collect()),
                "garden" => garden_name = Some(val.to_string().chars().filter(|c| *c != '\"').collect()),
                "location" => location = Some(val.to_string().parse::<f64>().unwrap()),
                "species" => species = Some(val.to_string().chars().filter(|c| *c != '\"').collect()),
                "variety" => variety = Some(val.to_string().chars().filter(|c| *c != '\"').collect()),
                _ => {
                    eprintln!("{} no option {}", "Err:".red(), key);
                    return;
                }
            }
        }
        else {
            eprintln!("{} no value for option {}", "Err:".red(), option[0]);
                    return;
        }
    }
    let garden_name = garden_name.unwrap_or_else(|| {prompt_input("Garden name: ")});
    let container_name = container_name.unwrap_or_else(|| {prompt_input("Container name: ")});
    let location = match location {
        Some(val) => Some(val),
        None => prompt_input_f64("location: ")
    };
    let species = species.unwrap_or_else(|| {prompt_input("Species: ")});
    let variety = variety.unwrap_or_else(|| {prompt_input("Variety: ")});

    println!("{}", "Adding plant".green());

    let new_plant = plant_controller::Plant::new(
        None, 
        None, 
        Some(container_name), 
        Some(garden_name), 
        location, 
        Some(species), 
        Some(variety), 
        None, 
        None
    );
    
    match new_plant.add(pool) {
        Ok(_) => {}
        Err(e) => eprint!("{}: {}", "Error".red(), e)
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

fn view_plants(pool: Pool, args: &Vec<String>) {
    let mut container_name = None;
    let mut garden_name = None;

    for arg in args.iter() {
        let option: Vec<&str> = arg.split('=').collect();

        if option.len() == 2{
            let (key, val) = (option[0], option[1]);

            match key {
                "container" => container_name = Some(val.to_string().chars().filter(|c| *c != '\"').collect()),
                "garden" => garden_name = Some(val.to_string().chars().filter(|c| *c != '\"').collect()),
                _ => {
                    eprintln!("{} no option {}", "Err:".red(), key);
                    return;
                }
            }
        }
        else {
            eprintln!("{} no value for option {}", "Err:".red(), option[0]);
                    return;
        }
    }

    let garden_name = garden_name.unwrap_or_else(|| {prompt_input("Garden name: ")});
    let container_name = container_name.unwrap_or_else(|| {prompt_input("Container name: ")});

    match plant_controller::get_plants_container(pool, container_name.clone(), garden_name.clone()) {
        Ok(plants) => println!("{} plants in container {}, from {}", plants.len(), container_name, garden_name),
        Err(e) => eprintln!("{}:, {:?}", "Err".red(), e)
    }
}

fn help() {
    let help = 
    r#"plant help: 
    useage:
        plant [command] [options]

    commands:
        view <plant_id>"#;

    println!("{help}");
}