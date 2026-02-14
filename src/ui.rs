use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;

use crate::ducks::*;

pub fn ui_plugin(app: &mut App) {
    app.add_systems(Startup, (load_font, enter_game).chain());

    app.add_systems(OnEnter(UiState::Game), spawn_game_ui);
    app.add_systems(
        FixedUpdate,
        update_duck_info.run_if(in_state(UiState::Game)),
    );

    app.insert_state(UiState::Preload);
}

#[derive(States, Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum UiState {
    Preload,
    MainMenu,
    Game,
}

fn enter_game(mut commands: Commands) {
    // commands.set_state(UiState::Game);
}

#[derive(Resource)]
struct UiFont(Handle<Font>);

#[derive(Component)]
struct UiElement;

#[derive(Component)]
struct MainDuckInfo;

fn load_font(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("SNPro-Regular.ttf");
    commands.insert_resource(UiFont(font));
}

fn clear_ui(mut commands: Commands, ui: Query<Entity, With<UiElement>>) {
    for e in ui {
        commands.entity(e).despawn();
    }
}

fn update_duck_info(
    ducks: Query<&Transform, (With<Duck>, Without<Duckling>)>,
    text: Query<&mut Text, With<MainDuckInfo>>,
) {
    let mut s = String::new();
    for duck in ducks {
        s += &format!("{:?}\n", duck);
    }

    for mut t in text {
        t.0 = s.clone();
    }
}

fn spawn_game_ui(mut commands: Commands, font: Res<UiFont>) {
    commands
        .spawn((
            UiElement,
            Node {
                width: percent(100.0),
                height: px(150.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(ZINC_950.with_alpha(0.3).into()),
        ))
        .with_child((
            Node {
                margin: UiRect::all(Val::Px(8.0)),
                border: UiRect::all(px(2.0)),
                ..default()
            },
            TextFont {
                font: font.0.clone(),
                font_size: 32.0,
                ..default()
            },
            MainDuckInfo,
            Text::new("Hello!"),
            BorderColor::all(RED_400),
        ));
}
