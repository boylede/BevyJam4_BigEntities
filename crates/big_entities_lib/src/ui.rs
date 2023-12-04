use std::process;

use bevy::{
    app::AppExit,
    asset::{LoadState, UntypedHandle},
    ecs::system::Command,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use smallvec::SmallVec;

use crate::states::GameState;
pub mod score;
pub mod fps;

#[derive(Component)]
pub struct UiCommands(SmallVec<[UiCommand; 2]>);

impl UiCommands {
    fn add_all(&self, commands: &mut Commands) {
        for c in self.0.iter() {
            commands.add(c.clone())
        }
    }
}

impl UiCommands {
    pub fn builder() -> UiCommands {
        UiCommands(SmallVec::new())
    }
    pub fn with(mut self, command: UiCommand) -> UiCommands {
        self.push(command);
        self
    }
    pub fn push(&mut self, command: UiCommand) {
        self.0.push(command)
    }
}

/// commands that can be queued when UI elements are interacted with
#[derive(Clone)]
pub enum UiCommand {
    SwitchState(GameState),
    Despawn(Entity),
    SwapUi(Entity, Entity),
    CaptureMouse,
    ReleaseMouse,
    Terminate,
}

/// Trigger an associated InteractionCommand
#[derive(Component)]
pub enum Trigger {
    /// trigger when the given assets are loaded
    ///
    /// this must use UntypedHandle to support any asset
    AssetsLoaded(SmallVec<[UntypedHandle; 2]>),
    /// trigger when the given scene assets are loaded
    ///
    /// this handle type is smaller, so more can be stored inline
    ScenesLoaded(SmallVec<[Handle<Scene>; 3]>),
}

impl Command for UiCommand {
    fn apply(self, world: &mut World) {
        use UiCommand as IC;
        match self {
            IC::SwitchState(next_state) => {
                let mut ns = world.resource_mut::<NextState<GameState>>();
                ns.set(next_state);
            }
            IC::SwapUi(a, b) => {
                if let Some(ec) = world.get_entity_mut(a) {
                    ec.despawn_recursive();
                    let mut em = world.entity_mut(b);
                    let mut v = em.get_mut::<Visibility>().unwrap();
                    *v = Visibility::Visible;
                }
            }
            IC::Despawn(entity) => world.entity_mut(entity).despawn_recursive(),
            IC::CaptureMouse => {
                let mut window_query = world.query::<(&mut Window, With<PrimaryWindow>)>();
                let Ok((mut window, _)) = window_query.get_single_mut(world) else {
                    warn!("expected single window");
                    return;
                };
                window.cursor.visible = false;
                window.cursor.grab_mode = CursorGrabMode::Locked;
            }
            IC::ReleaseMouse => {
                let mut window_query = world.query::<(&mut Window, With<PrimaryWindow>)>();
                let Ok((mut window, _)) = window_query.get_single_mut(world) else {
                    warn!("expected single window");
                    return;
                };
                window.cursor.visible = true;
                window.cursor.grab_mode = CursorGrabMode::None;
            }
            IC::Terminate => {
                let Some(mut exit_queue) = world.get_resource_mut::<Events<AppExit>>() else {
                    warn!("didn't have app exit events?");
                    process::exit(0);
                };
                exit_queue.send(AppExit);
            }
        }
    }
}

/// look for buttons with InteractionCommand component, and execute that command when button is clicked
pub fn button_clicked(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &UiCommands,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut commands: Commands,
) {
    for (interaction, mut color, mut border_color, ic) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::rgb(0.20, 0.15, 0.15).into();
                border_color.0 = Color::RED;

                ic.add_all(&mut commands);
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.15, 0.20, 0.15).into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = Color::rgb(0.15, 0.15, 0.15).into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

/// Check Trigger components and schedule associated InteractionCommand if necessary
pub fn trigger_check(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    triggers: Query<(&Trigger, &UiCommands)>,
) {
    fn loaded(ls: Option<LoadState>) -> bool {
        ls == Some(LoadState::Loaded)
    }
    for (trigger, command) in triggers.iter() {
        match trigger {
            Trigger::AssetsLoaded(assets) => {
                if assets
                    .iter()
                    .map(|handle| asset_server.get_load_state(handle.id()))
                    .all(loaded)
                {
                    command.add_all(&mut commands);
                }
            }
            Trigger::ScenesLoaded(scenes) => {
                if scenes
                    .iter()
                    .map(|handle| asset_server.get_load_state(handle.id()))
                    .all(loaded)
                {
                    command.add_all(&mut commands);
                }
            }
        }
    }
}
