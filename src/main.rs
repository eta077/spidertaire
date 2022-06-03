#![deny(missing_docs)]
//! An implementation of the card game Spider Solitaire.

use std::collections::HashMap;

use bevy::prelude::*;

use spidertaire::{Card, CardSuit, Deck};

/// An enumeration of the available difficulties.
/// Determines the number of suits in play.
#[allow(dead_code)] // TODO: determine level based on cache/previous game/user choice
#[derive(Debug, Clone, Copy)]
enum Difficulty {
    Easy,
    Medium,
    Hard,
}

/// A component representing a card.
#[derive(Component)]
struct CardGui {
    card: Card,
}

/// A component indicating the card is hidden (face down).
#[derive(Component)]
struct Hidden;

/// A component indicating the card is shown (face up).
#[derive(Component)]
struct Shown;

/// A component representing a card's position in the central grid.
#[derive(Component, Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct GridPosition {
    x: u8,
    y: u8,
}

impl GridPosition {
    /// Gets the position of the card below this one.
    pub fn next_row(&self) -> Self {
        GridPosition {
            x: self.x,
            y: self.y + 1,
        }
    }
}

#[derive(Component, Clone)]
struct Available {
    cards: [Card; 10],
}

type LegalMoves = Vec<(GridPosition, GridPosition)>;

const WINDOW_WIDTH: f32 = 960.0;
const WINDOW_HEIGHT: f32 = 540.0;
const SPACING: f32 = 5.0;
const SPACING_Y: f32 = 25.0;
const CARD_SIZE: f32 = (WINDOW_WIDTH - SPACING * 11.0) / 10.0;
const AVAILABLE_X: f32 = WINDOW_WIDTH / 2.0 - SPACING;
const AVAILABLE_Y: f32 = WINDOW_HEIGHT / 2.0 - SPACING - CARD_SIZE / 2.0;
const GRID_X: f32 = WINDOW_WIDTH / -2.0 + SPACING;
const GRID_Y: f32 = WINDOW_HEIGHT / 2.0 - SPACING - CARD_SIZE - SPACING;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: String::from("Spidertaire"),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .init_resource::<LegalMoves>()
        .add_startup_system(setup)
        .add_system_to_stage(CoreStage::PreUpdate, calculate_legal_moves)
        .add_system_to_stage(CoreStage::PreUpdate, handle_grid_input)
        .add_system_to_stage(CoreStage::PreUpdate, handle_available_input)
        .add_system_to_stage(CoreStage::PreUpdate, show_revealed_cards)
        .add_system(draw_available_sets)
        .add_system(draw_grid_hidden)
        .add_system(draw_grid_shown)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    new_game(commands, Difficulty::Easy);
}

fn new_game(mut commands: Commands, difficulty: Difficulty) {
    let mut deck = match difficulty {
        Difficulty::Easy => Deck::from_suit(CardSuit::Spades),
        Difficulty::Medium => Deck::from_suits(CardSuit::Spades, CardSuit::Hearts),
        Difficulty::Hard => Deck::new(),
    };
    deck.combine(deck.clone());
    deck.shuffle();
    let mut grid_cards = HashMap::<GridPosition, Entity>::with_capacity(54);
    for (pos, card) in deck.cards.drain(0..44).enumerate() {
        let position = GridPosition {
            x: (pos % 10) as u8,
            y: (pos / 10) as u8,
        };
        let entity = commands
            .spawn()
            .insert(CardGui { card })
            .insert(Hidden)
            .insert(position)
            .id();
        grid_cards.insert(position, entity);
    }
    for (pos, card) in deck.cards.drain(0..10).enumerate() {
        let position = GridPosition {
            x: ((pos + 44) % 10) as u8,
            y: ((pos + 44) / 10) as u8,
        };
        let entity = commands
            .spawn()
            .insert(CardGui { card })
            .insert(Shown)
            .insert(position)
            .id();
        grid_cards.insert(position, entity);
    }
    commands.insert_resource(grid_cards);

    let mut available_sets = Vec::with_capacity(5);
    for set in deck.cards.chunks_exact(10) {
        let available = Available {
            cards: set.to_owned().try_into().unwrap(),
        };
        commands.spawn().insert(available.clone());
        available_sets.push(available);
    }
    commands.insert_resource(available_sets);
}

fn calculate_legal_moves(
    mut legal_moves: ResMut<LegalMoves>,
    grid_cards: Res<HashMap<GridPosition, Entity>>,
    query: Query<(&CardGui, &Shown, &GridPosition)>,
) {
    let other_cards = query
        .iter()
        .collect::<Vec<(&CardGui, &Shown, &GridPosition)>>();
    let mut new_legal_moves = Vec::with_capacity(40);
    for card in other_cards.clone() {
        for other_card in &other_cards {
            if let Some(value) = other_card.0.card.value.next() {
                if value == card.0.card.value {
                    let new_position = other_card.2.next_row();
                    if grid_cards.get(&new_position).is_none() {
                        new_legal_moves.push((card.2.to_owned(), new_position));
                    }
                }
            }
        }
    }
    *legal_moves = new_legal_moves;
}

fn draw_available_sets(
    mut commands: Commands,
    query: Query<(Entity, &Available)>,
    assets: Res<AssetServer>,
) {
    for (pos, (e, _)) in query.iter().enumerate() {
        let x = AVAILABLE_X - ((CARD_SIZE / 2.0) * pos as f32) - (CARD_SIZE / 2.0);
        let y = AVAILABLE_Y;
        let scale = CARD_SIZE / 200.0;
        commands.entity(e).insert_bundle(SpriteBundle {
            texture: assets.load("textures/back.png"),
            transform: Transform {
                translation: Vec3::new(x, y, pos as f32),
                scale: Vec3::new(scale, scale, 1.0),
                ..default()
            },
            ..default()
        });
    }
}

fn draw_grid_hidden(
    mut commands: Commands,
    query: Query<(Entity, &Hidden, &GridPosition)>,
    assets: Res<AssetServer>,
) {
    for (e, _, pos) in query.iter() {
        let x = GRID_X + ((CARD_SIZE + SPACING) * pos.x as f32) + (CARD_SIZE / 2.0);
        let y = GRID_Y - (SPACING_Y * pos.y as f32) - (CARD_SIZE / 2.0);
        let scale = CARD_SIZE / 200.0;
        commands.entity(e).insert_bundle(SpriteBundle {
            texture: assets.load("textures/back.png"),
            transform: Transform {
                translation: Vec3::new(x, y, pos.y as f32),
                scale: Vec3::new(scale, scale, 1.0),
                ..default()
            },
            ..default()
        });
    }
}

fn draw_grid_shown(
    mut commands: Commands,
    query: Query<(Entity, &CardGui, &Shown, &GridPosition)>,
    assets: Res<AssetServer>,
) {
    for (e, card, _, pos) in query.iter() {
        let x = GRID_X + ((CARD_SIZE + SPACING) * pos.x as f32) + (CARD_SIZE / 2.0);
        let y = GRID_Y - (SPACING_Y * pos.y as f32) - (CARD_SIZE / 2.0);
        let color = match card.card.suit {
            CardSuit::Hearts | CardSuit::Diamonds => Color::RED,
            CardSuit::Clubs | CardSuit::Spades => Color::BLACK,
        };
        commands
            .entity(e)
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(CARD_SIZE, CARD_SIZE)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(x, y, pos.y as f32),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(Text2dBundle {
                    text: Text::with_section(
                        card.card.value.as_str(),
                        TextStyle {
                            font: assets.load("fonts/arial.ttf"),
                            font_size: 20.0,
                            color,
                        },
                        Default::default(),
                    ),
                    transform: Transform::from_xyz(
                        CARD_SIZE / -2.0,
                        CARD_SIZE / 2.0,
                        (pos.y + 1) as f32,
                    ),
                    ..Default::default()
                });
                parent.spawn_bundle(Text2dBundle {
                    text: Text::with_section(
                        card.card.suit.as_str(),
                        TextStyle {
                            font: assets.load("fonts/arial.ttf"),
                            font_size: 20.0,
                            color,
                        },
                        Default::default(),
                    ),
                    transform: Transform::from_xyz(
                        CARD_SIZE / 2.0 - 20.0,
                        CARD_SIZE / 2.0,
                        (pos.y + 1) as f32,
                    ),
                    ..Default::default()
                });
            });
    }
}

#[derive(Debug)]
struct Bounds2 {
    pub pos: Vec2,
    pub size: Vec2,
}

impl Bounds2 {
    pub fn contains(&self, coords: Vec2) -> bool {
        coords.x >= self.pos.x
            && coords.y >= self.pos.y
            && coords.x <= self.pos.x + self.size.x
            && coords.y <= self.pos.y + self.size.y
    }
}

fn handle_grid_input(
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    legal_moves: Res<LegalMoves>,
    mut grid_cards: ResMut<HashMap<GridPosition, Entity>>,
    mut query: Query<(&Shown, &mut GridPosition, &Transform)>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(press_location) = windows
            .get_primary()
            .and_then(|window| window.cursor_position())
        {
            let window_size = Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT);
            let world_press_location = press_location - window_size / 2.0;
            let mut successful_move = None;
            for (_, mut position, transform) in query.iter_mut() {
                let bounds = Bounds2 {
                    pos: Vec2::new(
                        transform.translation.x - CARD_SIZE / 2.0,
                        transform.translation.y - CARD_SIZE / 2.0,
                    ),
                    size: Vec2::new(CARD_SIZE, CARD_SIZE),
                };
                if bounds.contains(world_press_location) {
                    for legal_move in &(*legal_moves) {
                        if legal_move.0 == *position {
                            let entity = grid_cards
                                .remove(&position)
                                .expect("Grid cards and components are out of sync");
                            bevy::log::error!("moving {:?} to {:?}", position, legal_move.1);
                            grid_cards.insert(legal_move.1, entity);
                            *position = legal_move.1;
                            successful_move = Some(legal_move.to_owned());
                            break;
                        }
                    }
                    break;
                }
            }
            if let Some(input_move) = successful_move {
                for (_, mut position, _) in query.iter_mut() {
                    if position.x == input_move.0.x && position.y > input_move.0.y {
                        let y_diff = position.y - input_move.0.y;
                        let entity = grid_cards
                            .remove(&position)
                            .expect("Grid cards and components are out of sync");
                        let new_position = GridPosition {
                            x: input_move.1.x,
                            y: input_move.1.y + y_diff,
                        };
                        bevy::log::error!("moving {:?} to {:?}", position, new_position);
                        grid_cards.insert(new_position, entity);
                        *position = new_position;
                    }
                }
            }
        }
    }
}

fn handle_available_input(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut grid_cards: ResMut<HashMap<GridPosition, Entity>>,
    mut available_sets: ResMut<Vec<Available>>,
    query: Query<(Entity, &Available)>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        if available_sets.is_empty() {
            bevy::log::trace!("not adding available set because available sets is empty");
            return;
        }
        if let Some(press_location) = windows
            .get_primary()
            .and_then(|window| window.cursor_position())
        {
            let window_size = Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT);
            let world_press_location = press_location - window_size / 2.0;
            let bounds = Bounds2 {
                pos: Vec2::new(AVAILABLE_X - CARD_SIZE * 3.5, AVAILABLE_Y - CARD_SIZE / 2.0),
                size: Vec2::new(CARD_SIZE, CARD_SIZE),
            };
            if bounds.contains(world_press_location) {
                if let Some((e, _)) = query.iter().last() {
                    commands.entity(e).despawn();
                }
                let max_rows = find_max_rows(grid_cards.keys());
                bevy::log::error!("adding available set to max rows: {:?}", max_rows);
                for (pos, card) in available_sets.remove(0).cards.into_iter().enumerate() {
                    let position = GridPosition {
                        x: pos as u8,
                        y: max_rows[pos],
                    };
                    let entity = commands
                        .spawn()
                        .insert(CardGui { card })
                        .insert(Shown)
                        .insert(position)
                        .id();
                    grid_cards.insert(position, entity);
                }
            } else {
                bevy::log::trace!(
                    "not adding available set because {:?} does not contain {:?}",
                    bounds,
                    world_press_location
                );
            }
        } else {
            bevy::log::error!("unable to find cursor position in primary window");
        }
    }
}

fn find_max_rows<'a, T>(positions: T) -> [u8; 10]
where
    T: Iterator<Item = &'a GridPosition>,
{
    let mut result = [0; 10];
    for pos in positions {
        if pos.y >= result[pos.x as usize] {
            result[pos.x as usize] = pos.y + 1;
        }
    }
    result
}

fn show_revealed_cards(
    mut commands: Commands,
    grid_cards: Res<HashMap<GridPosition, Entity>>,
    query: Query<(Entity, &GridPosition, &Hidden)>,
) {
    for (e, position, _) in query.iter() {
        if grid_cards.get(&position.next_row()).is_none() {
            commands.entity(e).remove::<Hidden>().insert(Shown);
        }
    }
}
