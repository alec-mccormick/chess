use bevy::prelude::*;
use bevy_networking::{NetworkResource, NetworkingPlugin};
use clap::Clap;

use log::info;
use std::net::SocketAddr;

use chess::{
    core::{CorePlugin, AppConfig},
    render::RenderPlugin,
    ui::UIPlugin,
};


#[derive(Clap, Debug)]
#[clap(version = "3.0", author = "Alec McCormick <alecs.mccormick@gmail.com>")]
struct Opts {
    /// Local server port to use. Multiple clients running on the same machine must each use a unique port.
    #[clap(short, long, default_value = "12351")]
    pub port: String,

    /// Remote address for other client, Ex: 127.0.0.1:12350 to connect to a client running locally on port 12350.
    #[clap(short, long)]
    pub remote: Option<String>,

    /// Window Width
    #[clap(short, long, default_value = "1680")]
    pub width: u32,

    /// Window Width
    #[clap(short, long, default_value = "1050")]
    pub height: u32,

    /// Window Title
    #[clap(long, default_value = "Chess!")]
    pub title: String,
}


fn main() {
    env_logger::init();

    let opts: Opts = Opts::parse();

    let config = AppConfig {
        port: opts.port,
        remote_addr: opts.remote,
    };

    println!("App Config is: {:?}", config);
    println!("Launching game!");

    App::build()
        .add_resource(config)
        .add_resource(WindowDescriptor {
            title: opts.title,
            width: opts.width,
            height: opts.height,
            vsync: false,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(NetworkingPlugin)
        .add_plugin(CorePlugin)
        .add_plugin(UIPlugin)
        .add_plugin(RenderPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    config: Res<AppConfig>,
    mut net: ResMut<NetworkResource>,
    mut events: ResMut<Events<chess::ui::CreateMainMenuEvent>>,
) {
    net.bind(format!("0.0.0.0:{}", config.port)).unwrap();
    info!("App Setup - Spawning Main Menu");

    events.send(chess::ui::CreateMainMenuEvent);
    // commands.spawn((chess::ui::MainMenu,));
}
