use mysql::Pool;
use colored::Colorize;

use crate::database::container_controller;

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
        Ok(res) => println!("{:?}", res),
        Err(e) => eprintln!("{}:, {:?}", "Err".red(), e)
    }
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