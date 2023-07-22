use std::{collections::HashMap, env::current_exe, fs, path::PathBuf};

use macroquad::{
    prelude::FileError,
    texture::{load_texture, Texture2D},
};

const TEXTURE_PATHS: [&str; 1] = ["ui/splash.png"];

pub struct Textures {
    pub splash: Texture2D,
    pub planets: HashMap<String, Texture2D>,
}

impl Textures {
    pub fn get_asset_base_path(package: &str) -> PathBuf {
        let mut path = PathBuf::new();
        if let Ok(exe_path) = current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                path.push(exe_dir.to_path_buf());
            }
        }
        path.push(package);
        path.push("assets");

        return path;
    }

    /// Finds the path to a texture, based on the current executable's path.
    pub fn get_texture_path(package: &str, name: &str) -> String {
        let mut path = Self::get_asset_base_path(package);
        path.push(name);

        if let Some(path_str) = path.to_str() {
            return path_str.to_string();
        } else {
            return format!("{package}/assets/{name}");
        }
    }

    /// Loads a texture from its computed path.
    async fn load_texture_path(name: &str) -> Result<Texture2D, FileError> {
        let path = Self::get_texture_path("flight", name);
        load_texture(&path).await
    }

    async fn load_textures() -> Result<Vec<Texture2D>, FileError> {
        let mut textures: Vec<Texture2D> = vec![];
        for path in TEXTURE_PATHS {
            let texture = Self::load_texture_path(path).await?;
            textures.push(texture);
        }

        Ok(textures)
    }

    async fn load_planets() -> HashMap<String, Texture2D> {
        let mut planets: HashMap<String, Texture2D> = HashMap::new();
        let mut base = Self::get_asset_base_path("flight");
        base.push("planets");
        let dir = fs::read_dir(&base);
        if let Ok(dir) = dir {
            for planet_name in dir {
                if planet_name.is_err() {
                    log::warn!(
                        "Failed to read planet directory: {}",
                        planet_name.unwrap_err()
                    );
                    continue;
                }

                let planet_path = planet_name.unwrap().path();
                let planet_name = planet_path.to_str();
                if planet_name.is_none() {
                    log::warn!("Failed to read planet directory: path is not valid UTF-8.");
                    continue;
                }

                let texture = load_texture(&planet_name.unwrap()).await;
                if texture.is_err() {
                    log::warn!("Failed to load planet texture: {}", texture.unwrap_err());
                    continue;
                }

                let file_stem = planet_path.file_stem();
                if file_stem.is_none() {
                    log::warn!("Failed to load planet texture: path has no file stem.");
                    continue;
                }

                let texture = texture.unwrap();

                let file_stem = file_stem.unwrap().to_str().unwrap();
                planets.insert(file_stem.to_string(), texture);
            }
        }

        println!("{:?}", planets);

        planets
    }

    pub async fn new() -> Self {
        let textures = Self::load_textures().await;
        match textures {
            Ok(textures) => {
                log::info!("Loaded {} textures.", textures.len());

                let [splash] = textures[..] else {
                    // TODO: Find a better way to do this.
                    log::error!("Failed to load textures: not enough textures. This should not be possible.");
                    std::process::exit(1);
                };

                let planets = Self::load_planets().await;

                Self { splash, planets }
            }
            Err(err) => {
                log::error!("Failed to load textures: {}", err);
                std::process::exit(1);
            }
        }
    }
}
