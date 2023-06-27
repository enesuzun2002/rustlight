use rusqlite::Error;
use std::collections::HashMap;
use std::path::PathBuf;

mod db;
mod drive;

fn main() {
    // Retrieve installed applications from cache or directory
    let installed_apps: HashMap<String, PathBuf> = match db::retrieve_installed_apps_from_cache() {
        Ok(apps) => {
            if apps.is_empty() || drive::should_recache() {
                cache_apps()
            } else {
                apps
            }
        }
        Err(_) => cache_apps(),
    };

    for (app_name, app_path) in installed_apps {
        println!("{}: {}", app_name, app_path.display());
    }
}

fn cache_apps() -> HashMap<String, PathBuf> {
    let apps: HashMap<String, PathBuf> = drive::retrieve_installed_apps_from_directory().unwrap();
    let res: Result<(), Error> = db::save_installed_apps_to_cache(&apps);
    match res {
        Ok(()) => {
            println!("Successfully saved installed apps to cache.");
        }
        Err(err) => {
            println!("Error while saving installed apps to cache: {}", err);
        }
    }
    apps
}
