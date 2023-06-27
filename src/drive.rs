use std::{collections::HashMap, ffi::OsString, os::windows::prelude::OsStringExt, path::PathBuf, time::{SystemTime, Duration}, fs};
use walkdir::{Error, WalkDir};
use windows::Win32::Storage::FileSystem::GetLogicalDriveStringsW;

pub fn list_drives() -> Vec<OsString> {
    let size = unsafe { GetLogicalDriveStringsW(Some(&mut [])) };
    let mut buffer = vec![0; size as usize];
    unsafe { GetLogicalDriveStringsW(Some(&mut buffer)) };
    let mut list: Vec<_> = buffer
        .split(|x| *x == 0)
        .map(OsStringExt::from_wide)
        .collect();
    list.truncate(list.len() - 2);
    let mut additional_drives = list.clone();
    // Filter out the "C:\" drive
    additional_drives.retain(|drive: &OsString| drive.to_string_lossy() != "C:\\");
    additional_drives
}

pub fn retrieve_installed_apps_from_directory() -> Result<HashMap<String, PathBuf>, Error> {
    let mut installed_apps: HashMap<String, PathBuf> = HashMap::new();

    let additional_drives = list_drives();

    // Check common installation directories
    let installation_directories = [
        "C:\\Program Files",
        "C:\\Program Files (x86)",
        &format!("C:\\Users\\{}\\AppData\\Local", whoami::username()),
        &format!("C:\\Users\\{}\\AppData\\Roaming", whoami::username()),
    ];

    for directory in installation_directories.iter() {
        for entry in WalkDir::new(directory) {
            if let Ok(entry) = entry {
                if entry.file_type().is_file() && entry.path().extension().is_some() {
                    if entry.path().extension().unwrap() == "exe" {
                        if let Some(file_name) = entry.path().file_name().and_then(|n| n.to_str()) {
                            installed_apps
                                .insert(file_name.to_string(), entry.path().to_path_buf());
                        }
                    }
                }
            }
        }
    }

    for directory in additional_drives.iter() {
        for entry in WalkDir::new(directory) {
            if let Ok(entry) = entry {
                if entry.file_type().is_file() && entry.path().extension().is_some() {
                    if entry.path().extension().unwrap() == "exe" {
                        if let Some(file_name) = entry.path().file_name().and_then(|n| n.to_str()) {
                            installed_apps
                                .insert(file_name.to_string(), entry.path().to_path_buf());
                        }
                    }
                }
            }
        }
    }

    Ok(installed_apps)
}

pub fn should_recache() -> bool {
    // Check if the data in the cache is older than 3 days
    let should_recache = match fs::metadata("rustlight.db") {
        Ok(metadata) => {
            let modified = metadata.modified().unwrap();
            let elapsed = SystemTime::now()
                .duration_since(modified)
                .unwrap_or(Duration::from_secs(0));
            elapsed > Duration::from_secs(3 * 24 * 60 * 60) // 3 days in seconds
        }
        Err(_) => true, // Cache doesn't exist, recache
    };
    should_recache
}


