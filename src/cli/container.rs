use std::io::Write;

use mysql::Pool;
use colored::Colorize;

use crate::database::container_controller::{self, ContainerController};
use crate::cli::{prompt_input, prompt_input_f64};

pub fn container_cmd(args: &Vec<String>, pool: Pool) {
    let cmd = if args.len() > 0 {args[0].clone()} else {format!("no command entered")};
    match cmd.as_str() {
        "view" => {
            let mut args_iter = args.iter();
            args_iter.next();
            view_container(pool, args_iter.next().unwrap_or(&String::new()), args_iter.next().unwrap_or(&String::new()));
        },
        "add" => add_container(pool, &args[1..].to_vec()),
        "help" => help(),
        _ => eprintln!("{} command '{cmd}' not found for container. Run 'container help' to see availavle commands", "Error:".red())
    }
}

fn view_container(pool: Pool, container_name: &String, garden_name: &String) {
    if garden_name.len() == 0 && container_name.len() > 0 {
        //if garden_name is zero than treat container_name as garden instead
        view_containers(pool, container_name);
        return;
    }
    else if garden_name.len() == 0 && container_name.len() == 0 {
        eprintln!("{}: {}", "Error".red(), "expected a command like 'container view [container name] <garden name>'");
        return;
    }

    match container_controller::view_container(pool, container_name, garden_name) {
        Ok(res) => {
            for container in res.iter() {
                display_container_img(container.clone(), None);
            }
        },
        Err(e) => eprintln!("{}:, {:?}", "Err".red(), e)
    }
}

fn view_containers(pool: Pool, garden_name: &String) {
    match container_controller::view_containers(pool, garden_name) {
        Ok(res) => {
            for container in res.iter() {
                display_container_img(container.clone(), None);
            }
        },
        Err(e) => eprintln!("{}:, {:?}", "Err".red(), e)
    }
}

//default adds raised bed
fn add_container(pool: Pool, args: &Vec<String>) {
    println!("{}", "Enter container info".green());

    let mut garden_name = None;
    let mut container_name = None;
    let mut length = None;
    let mut width = None;
    let mut height = None;
    let mut volume = None;

    for arg in args.iter() {
        let option: Vec<&str> = arg.split('=').collect();

        if option.len() == 2{
            let (key, val) = (option[0], option[1]);

            match key {
                "garden" => garden_name = Some(val.to_string().chars().filter(|c| *c != '\"').collect()),
                "container" => container_name = Some(val.to_string().chars().filter(|c| *c != '\"').collect()),
                "length" => length = Some(val.to_string().parse::<f64>().unwrap()),
                "width" => width = Some(val.to_string().parse::<f64>().unwrap()),
                "height" => height = Some(val.to_string().parse::<f64>().unwrap()),
                "volume" => volume = Some(val.to_string().parse::<f64>().unwrap()),
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
    let length = match length {
        Some(val) => Some(val),
        None => prompt_input_f64("length: ")
    };
    let width = match width {
        Some(val) => Some(val),
        None => prompt_input_f64("width: ")
    };
    let height = match height {
        Some(val) => Some(val),
        None => prompt_input_f64("height: ")
    };
    let volume = match volume {
        Some(val) => Some(val),
        None => prompt_input_f64("volume: ")
    };

    println!("{}", "Adding container".green());

    let new_container = container_controller::ContainerController::new(
        container_name, 
        garden_name, 
        None, 
        width, 
        length, 
        height, 
        volume
    );

    match new_container.add_container_bed(pool){
        Ok(_) => display_container_img(new_container, None),
        Err(e) => eprintln!("{}:, {:?}", "Err".red(), e)
    };
}

fn display_container_img(container: ContainerController, scaler: Option<f64>){
    let container_name = container.container_name();
    let garden_name = container.garden_name();
    let container_str = format!("Container: {container_name}\nGarden: {garden_name}");

    println!("{}", container_str);
    crate::cli::container_img::ContainerIMG::new(&container, scaler)
        .img_str();
} 

fn help() {
    let help = 
    r#"container help: 
    useage:
        container [command] [options]

    commands:
        view [container_name] <garden name>
        add [<options>]
            options:
                garden=<garden name>
                container=<new container name>
                lenght=<length in inches>
                height=<height in inches>
                width=<width in inches>
                volume=<volume in inches>"#;

    println!("{help}");
}