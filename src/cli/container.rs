use mysql::Pool;
use colored::Colorize;

use crate::database::container_controller::{self, ContainerController};

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

fn display_container_img(container: ContainerController, scaler: Option<f64>){
    let container_name = container.container_name();
    let garden_name = container.garden_name();
    let container_str = format!("Container: {container_name}\nGarden: {garden_name}");

    println!("{}", container_str);
    crate::cli::container_img::ContainerIMG::new(&container, None)
        .img_str();
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