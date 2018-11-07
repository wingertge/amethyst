//! An example showing how to create a dispatcher inside of a State.

extern crate amethyst;

use std::marker::PhantomData;
use amethyst::ecs::{Dispatcher, DispatcherBuilder};
use amethyst::prelude::*;
use amethyst::Error;

struct StateA;

impl SimpleState<'static, 'static> for StateA {
    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans<'static, 'static> {
        println!("StateA::update()");
        // Shows how to push a `Trans` through the event queue.
        // If you do use TransQueue, you will be forced to use the 'static lifetime on your states.
        data.world
            .write_resource::<TransQueue<GameData<'static, 'static>, StateEvent>>()
            .push_back(Box::new(|| Trans::Push(Box::new(StateB::default()))));
        Trans::Push(Box::new(StateB::default()))
    }
}

/// StateB isn't Send + Sync
struct StateB<'a> {
    dispatcher: Dispatcher<'static, 'static>,
    _phantom: &'a PhantomData<()>,
}

impl<'a> Default for StateB<'a> {
    fn default() -> Self {
        StateB {
            dispatcher: DispatcherBuilder::new().build(),
            _phantom: &PhantomData,
        }
    }
}

impl<'a> SimpleState<'static, 'static> for StateB<'a> {
    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans<'static, 'static> {
        println!("StateB::update()");
        self.dispatcher.dispatch(&mut data.world.res);
        Trans::Quit
    }
}

fn main() -> Result<(), Error> {
    amethyst::start_logger(Default::default());
    let mut game = Application::build("./", StateA)?.build(GameDataBuilder::default())?;
    game.run();
    Ok(())
}
