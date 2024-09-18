use mysql::Pool;
use colored::Colorize;

use crate::database::garden_controller;

pub fn garden_cmd(args: &Vec<String>, pool: Pool) {
    //default to view command if no commands passed
    if args.len() == 0 {
        view_gardens(pool, &"".to_string());

        return;
    }

    //check command if exists
    let cmd = args[0].clone();
    match cmd.as_str() {
        "view" => view_gardens(pool, args.iter().nth(1).unwrap_or(&"".to_string())),
        "add" => add_garden(pool, args.iter().nth(1).unwrap_or(&"".to_string())),
        "rm" => rm_garden(pool, args.iter().nth(1).unwrap_or(&"".to_string())),
        "new-name" => {
            let mut args_iter = args.iter();
            args_iter.next();
            new_name(pool, args_iter.next().unwrap_or(&"".to_string()), args_iter.next().unwrap_or(&"".to_string()))
        },
        "help" => help(),
        _ => eprintln!("{} command '{cmd}' not found for garden. Run 'garden help; to see availavle commands", "Error:".red())
    }
}

fn view_gardens(pool: Pool, gname: &String) {
    if gname.len() != 0 {
        match garden_controller::view_garden_containers(pool, &gname) {
            Ok(res) => println!("{:?}", res),
            Err(e) => eprintln!("{:?}", e)
        }

        return;
    }

    match garden_controller::view_gardens(pool) {
        Ok(res) => println!("{:?}", res),
        Err(e) => eprintln!("{:?}", e)
    };
}

fn add_garden(pool: Pool, gname: &String) {
    if gname.len() == 0 {
        eprintln!("{}: no name for garden add. Expected garden add <name>", "Error".red());

        return;
    }

    match garden_controller::add_garden(pool, gname) {
        Ok(_) => {},
        Err(e) => eprintln!("{}: {:?}", "Error".red(), e)
    }
}

fn rm_garden(pool: Pool, name: &String) {
    if name.len() == 0 {
        eprintln!("{}: no name for garden rm. Expected: `garden rm <name>`", "Error".red());

        return;
    }

    match garden_controller::remove_garden(pool, name) {
        Ok(_) => {},
        Err(e) => eprintln!("{}: {:?}", "Error".red(), e)
    }
}

fn new_name(pool: Pool, old_name: &String, new_name: &String) {
    if old_name.len() == 0 || new_name.len() == 0 {
        eprintln!("{}: Two arguments expected for garden new-name. Expected: `garden new-name <old_name> <new_name>`", "Error".red());

        return;
    }

    match garden_controller::new_name_garden(pool, old_name, new_name) {
        Ok(_) => {},
        Err(e) => eprintln!("{}: {:?}", "Error".red(), e)
    }
}

fn help() {
    let help = 
    r#"garden help: 
    useage:
        garden [command] [options]

    commands:
        view [garden name]
        add <garden name>
        rm <garden name>
        new-name <old name> <new name>"#;

    println!("{help}");
}