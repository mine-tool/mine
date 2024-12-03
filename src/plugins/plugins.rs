// Modrinth API

use modrinth_api::models::Project;

pub async fn search_plugin(name: String) -> Result<Project, Box<dyn std::error::Error>> {
    let configuration = modrinth_api::apis::configuration::Configuration::new();

    let project = modrinth_api::apis::projects_api::get_project(&configuration, &name).await.unwrap();
    Ok(project)
}