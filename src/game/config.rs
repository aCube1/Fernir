use ggez::{
    conf::{Backend, Conf, FullscreenType, NumSamples, WindowMode, WindowSetup},
    error::GameError,
    ContextBuilder, GameResult,
};
use log::*;
use std::{env, fs::File, io::Write, path};

pub fn load_config(ctx_builder: ContextBuilder) -> GameResult<ContextBuilder> {
    let mut conf = Conf::new()
        .window_mode(
            WindowMode::default()
                .fullscreen_type(FullscreenType::Windowed)
                .resizable(false)
                .dimensions(800.0, 600.0),
        )
        .backend(Backend::default().version(3, 3));
    conf.window_setup = WindowSetup {
        title: "Fernir".into(),
        samples: NumSamples::One,
        vsync: true,
        icon: "".into(),
        srgb: true,
    };

    let mut resource_path: path::PathBuf;

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        resource_path = path::PathBuf::from(manifest_dir.clone());
        resource_path.push("assets");

        if !resource_path.is_dir() {
            return Err(GameError::FilesystemError(
                "Cannot Open Assets Directory".to_string(),
            ));
        }

        let mut conf_path = path::PathBuf::from(manifest_dir);
        conf_path.push("conf.toml");

        if !conf_path.is_file() {
            warn!("Cannot Open 'conf.toml', Creating a New One...");
            let mut conf_file = File::create(conf_path)?;
            conf.to_toml_file(&mut conf_file)?;
            conf_file.flush()?;
        } else {
            let mut conf_file = File::open(conf_path)?;
            conf = Conf::from_toml_file(&mut conf_file)?;
        }
    } else {
        return Err(GameError::FilesystemError(
            "Cannot Get Manifest Dir".to_string(),
        ));
    }

    Ok(ctx_builder
        .default_conf(conf)
        .add_resource_path(resource_path))
}
