//! Displays a shaded sphere to the user.

use amethyst::{assets::{PrefabLoaderSystemDesc, Processor}, audio::{Source}, core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle, Time}, derive::SystemDesc, ecs::prelude::{Entity, System, SystemData, WorldExt, Write}, input::{InputBundle, StringBindings}, prelude::*, renderer::{
    plugins::RenderToWindow,
    rendy::mesh::{Normal, Position, TexCoord},
    types::DefaultBackend,
    RenderingBundle,
}, shrev::{EventChannel, ReaderId}, ui::{
    Anchor, RenderUi, UiBundle, UiEvent, UiFinder, UiText,
}, utils::{
    application_root_dir,
    fps_counter::{FpsCounter},
    scene::BasicScenePrefab,
}, LoggerConfig};
use log::{info, LevelFilter};
use amethyst_ui::{UiTransform, UiMultipartText, TextSection, get_default_font, FontAsset, LineMode};
use amethyst_assets::AssetStorage;
use amethyst::assets::Loader;

type MyPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;

#[derive(Default)]
struct Example {
    fps_display: Option<Entity>,
    random_text: Option<Entity>,
}

impl SimpleState for Example {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { mut world, .. } = data;

        let font = {
            let loader = world.fetch::<Loader>();
            let storage = world.fetch::<AssetStorage<FontAsset>>();
            get_default_font(&loader, &storage)
        };
        let transform = UiTransform::new(
            "multipart_text".to_string(),
            Anchor::Middle,
            Anchor::Middle,
            0.0,
            0.0,
            0.0,
            500.0,
            500.0
        );
        let text = UiMultipartText::new(
            vec![TextSection {
                text: "This next part will be colored ".to_string(),
                color: [0.0, 0.0, 0.0, 1.0],
                font: font.clone(),
                font_size: 21.0
            }, TextSection {
                text: "green".to_string(),
                color: [0.0, 1.0, 0.0, 1.0],
                font: font.clone(),
                font_size: 21.0
            }, TextSection {
                text: " and this part will be ".to_string(),
                color: [0.0, 0.0, 0.0, 1.0],
                font: font.clone(),
                font_size: 21.0
            }, TextSection {
                text: "twice as big!".to_string(),
                color: [0.0, 0.0, 0.0, 1.0],
                font,
                font_size: 42.0
            }],
            LineMode::Wrap,
            Anchor::Middle
        );
        world
            .create_entity()
            .with(transform)
            .with(text)
            .build();
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = state_data;

        if self.fps_display.is_none() {
            world.exec(|finder: UiFinder<'_>| {
                if let Some(entity) = finder.find("fps") {
                    self.fps_display = Some(entity);
                }
            });
        }
        if self.random_text.is_none() {
            world.exec(|finder: UiFinder| {
                if let Some(entity) = finder.find("random_text") {
                    self.random_text = Some(entity);
                }
            });
        }

        let mut ui_text = world.write_storage::<UiText>();
        {
            if let Some(fps_display) = self.fps_display.and_then(|entity| ui_text.get_mut(entity)) {
                if world.read_resource::<Time>().frame_number() % 20 == 0 {
                    let fps = world.read_resource::<FpsCounter>().sampled_fps();
                    fps_display.text = format!("FPS: {:.*}", 2, fps);
                }
            }
        }

        {
            if let Some(random_text) = self.random_text.and_then(|entity| ui_text.get_mut(entity)) {
                if let Ok(value) = random_text.text.parse::<i32>() {
                    let mut new_value = value * 10;
                    if new_value > 100_000 {
                        new_value = 1;
                    }
                    random_text.text = new_value.to_string();
                } else {
                    random_text.text = String::from("1");
                }
            }
        }

        Trans::None
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(LoggerConfig {
        level_filter: LevelFilter::Debug,
        ..Default::default()
    });

    let app_root = application_root_dir()?;

    let display_config_path = app_root.join("examples/ui/config/display.ron");
    let assets_dir = app_root.join("examples/ui/assets");

    let game_data = GameDataBuilder::default()
        .with_system_desc(PrefabLoaderSystemDesc::<MyPrefabData>::default(), "", &[])
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(Processor::<Source>::new(), "source_processor", &[])
        .with_system_desc(UiEventHandlerSystemDesc::default(), "ui_event_handler", &[])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderUi::default()),
        )?;

    let mut game = Application::build(assets_dir, Example::default())?
        // Unlimited FPS
        .with_frame_limit(FrameRateLimitStrategy::Unlimited, 9999)
        .build(game_data)?;
    game.run();
    Ok(())
}

/// This shows how to handle UI events.
#[derive(SystemDesc)]
#[system_desc(name(UiEventHandlerSystemDesc))]
pub struct UiEventHandlerSystem {
    #[system_desc(event_channel_reader)]
    reader_id: ReaderId<UiEvent>,
}

impl UiEventHandlerSystem {
    pub fn new(reader_id: ReaderId<UiEvent>) -> Self {
        Self { reader_id }
    }
}

impl<'a> System<'a> for UiEventHandlerSystem {
    type SystemData = Write<'a, EventChannel<UiEvent>>;

    fn run(&mut self, events: Self::SystemData) {
        // Reader id was just initialized above if empty
        for ev in events.read(&mut self.reader_id) {
            info!("[SYSTEM] You just interacted with a ui element: {:?}", ev);
        }
    }
}
