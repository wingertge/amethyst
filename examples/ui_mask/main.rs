use amethyst::assets::{HotReloadBundle, Loader};
use amethyst::core::TransformBundle;
use amethyst::input::{InputBundle, StringBindings};
use amethyst::prelude::{Builder, WorldExt};
use amethyst::renderer::types::DefaultBackend;
use amethyst::renderer::{RenderToWindow, RenderingBundle};
use amethyst::ui::{Anchor, Mask, RenderUi, UiBundle};
use amethyst::utils::application_root_dir;
use amethyst::{Application, GameData, GameDataBuilder, SimpleState, StateData};
use amethyst_assets::AssetStorage;
use amethyst_rendy::{ImageFormat, Texture};
use amethyst_ui::{UiImage, UiTransform};
use amethyst_core::Parent;

pub fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    // this will be the directory the 'Cargo.toml' is defined in.
    let app_root = application_root_dir()?;

    // our display config is in our configs folder.
    let display_config_path = app_root.join("examples/states_ui/config/display.ron");

    // other assets ('*.ron' files, '*.png' textures, '*.ogg' audio files, ui prefab files, ...) are here
    let assets_dir = app_root.join("examples/states_ui/assets");

    let game_data = GameDataBuilder::default()
        // a lot of other bundles/systems depend on this (without it being explicitly clear), so it
        // makes sense to add it early on
        .with_bundle(TransformBundle::new())?
        // This system is in 'events.rs'. Basically, it registers UI events that
        // happen. Without it, the buttons will not react.
        .with_bundle(InputBundle::<StringBindings>::new())?
        // this bundle allows us to 'find' the Buttons and other UI elements later on
        .with_bundle(UiBundle::<StringBindings>::new())?
        // this allows us to reload '*.ron' files during execution
        .with_bundle(HotReloadBundle::default())?
        // Without this, we would not get a picture.
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // This creates the window and draws a background, if we don't specify a
                // background in the loaded ui prefab file.
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.005, 0.005, 0.005, 1.0]),
                )
                // Without this, all of our beautiful UI would not get drawn.
                // It will work, but we won't see a thing.
                .with_plugin(RenderUi::default()),
            // If you want to draw Sprites and such, you would need this additionally:
            // .with_plugin(RenderFlat2D::default())
        )?;

    // creating the Application with the assets_dir, the first Screen, and the game_data with it's
    // systems.
    let mut game = Application::new(assets_dir, State, game_data)?;
    log::info!("Starting with WelcomeScreen!");
    game.run();

    Ok(())
}

#[derive(Debug, Clone, Copy, Default)]
struct State;

impl SimpleState for State {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let container = world
            .create_entity()
            .with(UiTransform::new(
                "container".to_string(),
                Anchor::Middle,
                Anchor::Middle,
                200.0,
                200.0,
                1.0,
                200.0,
                200.0,
            ))
            .with(UiImage::SolidColor([1.0, 0.0, 0.0, 1.0]))
            .build();

        let logo = {
            let loader = world.fetch::<Loader>();
            let textures = world.fetch::<AssetStorage<Texture>>();
            let handle = loader.load("texture/logo.png", ImageFormat::default(), (), &textures);
            UiImage::Texture(handle)
        };

        world
            .create_entity()
            .with(UiTransform::new(
                "logo".to_string(),
                Anchor::Middle,
                Anchor::Middle,
                70.0,
                70.0,
                1.0,
                300.0,
                300.0,
            ))
            .with(logo)
            .with(Mask { to: container })
            .with(Parent { entity: container })
            .build();
    }
}
