use std::{
    fs::File,
    io::{BufReader, Read},
};

enum ModLoader {
    Fabric,
    Quilt,
}

impl std::fmt::Display for ModLoader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModLoader::Fabric => write!(f, "Fabric"),
            ModLoader::Quilt => write!(f, "Quilt"),
        }
    }
}

fn main() -> anyhow::Result<(), anyhow::Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    for file in std::fs::read_dir(format!("{}/.minecraft/mods", std::env::var("HOME")?))? {
        let file = file?;
        if file.file_type()?.is_dir() {
            continue;
        }
        let mod_file_name = format!("{:?}", file.file_name());
        log::info!("updating {mod_file_name}");
        let path = &file.path();
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut archive = zip::ZipArchive::new(reader)?;

        let mut files = (None, None);

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            if file.name().contains('/') {
                continue;
            }

            if file.name() == "fabric.mod.json" {
                let mut buf = String::new();
                file.read_to_string(&mut buf)?;
                files.0 = Some(buf);
            } else if file.name() == "quilt.mod.json" {
                let mut buf = String::new();
                file.read_to_string(&mut buf)?;
                files.1 = Some(buf);
            }
        }

        let mod_loader;
        let file_content;

        match files {
            (Some(_), Some(_)) => {
                log::warn!(
                    "skipping: not yet able to update mods that support both fabric and quilt"
                );
                continue;
            }
            (Some(f), None) => {
                mod_loader = ModLoader::Fabric;
                file_content = f;
            }
            (None, Some(q)) => {
                mod_loader = ModLoader::Quilt;
                file_content = q;
            }
            (None, None) => {
                log::warn!("skipping: not a mod: does not contain any supported metadata files");
                continue;
            }
        }

        let mod_json: serde_json::Value = serde_json::from_str(&file_content)?;
        let mod_name = match mod_json["name"].as_str() {
            Some(mod_name) => mod_name,
            None => {
                log::warn!("skipping: missing a name attribute");
                continue;
            }
        };

        log::info!("finding {mod_loader} download for {mod_name} on modrinth");

        todo!();
    }

    Ok(())
}
