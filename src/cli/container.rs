use mysql::{binlog::row, Pool};
use colored::Colorize;

use crate::database::container_controller::{self, ContainerController};

const INCHS_IN_FEET: usize = 12;

pub fn container_cmd(args: &Vec<String>, pool: Pool) {
    let cmd = if args.len() > 0 {args[0].clone()} else {format!("no command entered")};
    match cmd.as_str() {
        "view" => {
            let mut args_iter = args.iter();
            args_iter.next();
            view_container(pool, args_iter.next().unwrap_or(&String::new()), args_iter.next().unwrap_or(&String::new()));
        },
        "help" => help(),
        _ => eprintln!("{} command '{cmd}' not found for container. Run 'container help; to see availavle commands", "Error:".red())
    }
}

fn view_container(pool: Pool, container_name: &String, garden_name: &String) {
    if container_name.len() == 0 || garden_name.len() == 0 {
        eprintln!("{}: {}", "Error".red(), "expected a command like 'container view <container name> <garden name>'");
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

fn display_container_img(container: ContainerController, scaler: Option<usize>){
    let scaler = scaler.unwrap_or(1);
    
    let container_name = container.container_name();
    let garden_name = container.garden_name();
    let mut container_str = format!(
        "\tContainer: {container_name}
        Garden: {garden_name}\n");

    //format rows
    //double row length for better formating
    let row_len = 2*(scaler * container.length() as usize) / INCHS_IN_FEET;

    let mut top_bottom_row = String::new();
    for _ in 0..row_len {
        top_bottom_row.push('+');
    }
    top_bottom_row.push('\n');

    let mut row = String::new();
    row.push('|');
    for _ in 0..row_len-2 {
        row.push(' ');
    }
    row.push('|');
    row.push('\n');

    //add colums to container_str
    let col_len = (scaler * container.width() as usize)/ INCHS_IN_FEET;
    container_str.push_str(&top_bottom_row);
    for _ in 0..col_len {
        container_str.push_str(&row);
    }
    container_str.push_str(&top_bottom_row);

    println!("{}", container_str);
} 

fn help() {
    let help = 
    r#"container help: 
    useage:
        container [command] [options]

    commands:
        view <container_name> <garden name>"#;

    println!("{help}");
}