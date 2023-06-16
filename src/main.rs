use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct ImageInfo {
    filename: String,
    title: String,
    path: String,
}

#[derive(Serialize, Deserialize)]
struct DirectoryInfo {
    dir: Vec<String>,
    images: Vec<ImageInfo>,
}

fn create_index_json(dir_path: &Path, prefix: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut dir_info = DirectoryInfo {
        dir: Vec::new(),
        images: Vec::new(),
    };

    let entries = fs::read_dir(dir_path)?;

    for entry in entries {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            let dir_name = entry_path.file_name().unwrap().to_string_lossy().to_lowercase().to_string();

            create_index_json(&entry_path, &format!("{}/{}", prefix, dir_name))?;

            dir_info.dir.push(dir_name);
        } else if let Some(extension) = entry_path.extension() {
            let extension = extension.to_string_lossy().to_lowercase();

            if extension == "svg" || extension == "jpg" || extension == "png" || extension == "webp" || extension == "gif" {
                let filename = entry_path.file_name().unwrap().to_string_lossy().to_string();
                let title = filename
                    .trim_end_matches(&format!(".{}", extension))
                    .replace("-", " ")
                    .replace("_", " ")
                    .to_lowercase();
                let path = format!("{}/{}", prefix, entry_path.file_name().unwrap().to_string_lossy());

                let image_info = ImageInfo {
                    filename,
                    title,
                    path,
                };

                dir_info.images.push(image_info);
            }
        }
    }

    let json_path = dir_path.join("index.json");

    let json_content = serde_json::to_string_pretty(&dir_info)?;
    fs::write(json_path, json_content)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir_path = PathBuf::from("icons");
    create_index_json(&dir_path, "")?;

    println!("Index JSON files created successfully.");

    Ok(())
}
