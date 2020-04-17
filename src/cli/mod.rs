#[allow(dead_code)]
#[allow(unused)]
mod config;
mod download;
mod github;
mod template;
mod zip;

pub fn update_cli() {
    // Check for updates
    template::update();
    println!("You are currently are up to date")
}

pub fn new_project(name: String) {
    println!("Generating {}", name);
    // Check for updates
    template::update();
    // Create project
    match template::new_project(name.as_str()) {
        Ok(_) => println!("Project successfully created"),
        Err(e) => {
            log::error!("unable to save project template because {}", e.to_string());
        }
    }
}

pub fn upgrade_project() {
    todo!("upgrade existing project");
}

pub fn serve_project() {
    todo!("start serving existing project");
}

pub fn watch_project() {
    todo!("start serving existing project, watch for changes and reload resources");
}
