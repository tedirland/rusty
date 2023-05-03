// init rusty_engine
use rusty_engine::prelude::*;

struct GameState {
    high_score: u32,
    score: u32,
    sweatpants_index: i32,
    // spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            high_score: 0,
            score: 0,
            sweatpants_index: 0,
            // spawn_timer: Timer::from_seconds(1.0, false),
        }
    }
}

fn main() {
    // create a mutable Game struct to keep track of state
    let mut game = Game::new();

    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
    player.translation = Vec2::new(30.0, 0.0);
    player.rotation = SOUTH_WEST;
    player.scale = 1.0;
    player.collision = true;

    let score = game.add_text("score", "Score: 0");
    score.translation = Vec2::new(520.0, 320.0);

    let high_score = game.add_text("high_score", "High Score: 0");
    high_score.translation = Vec2::new(-520.0, 320.0);

    // setup game here
    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // your actual game logic
    for event in engine.collision_events.drain(..) {
        println!("{:?}", event);
        if event.state == CollisionState::Begin && event.pair.one_starts_with("player") {
            // remove the sprite
            for label in [event.pair.0, event.pair.1] {
                if label != "player" {
                    engine.sprites.remove(&label);
                }
            }
            game_state.score += 1;
            let score = engine.texts.get_mut("score").unwrap();
            score.value = format!("Score {}", game_state.score);
            if game_state.score > game_state.high_score {
                game_state.high_score = game_state.score;
                let high_score = engine.texts.get_mut("high_score").unwrap();
                high_score.value = format!("High Score: {}", game_state.score);
            }
        }
    }

    // handle movement
    let player = engine.sprites.get_mut("player").unwrap();
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Up, KeyCode::W])
    {
        const MOVEMENT_SPEED: f32 = 100.0;
        player.translation.y += MOVEMENT_SPEED * engine.delta_f32;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Down, KeyCode::S])
    {
        const MOVEMENT_SPEED: f32 = 100.0;
        player.translation.y -= MOVEMENT_SPEED * engine.delta_f32;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Left, KeyCode::A])
    {
        const MOVEMENT_SPEED: f32 = 100.0;
        player.translation.x -= MOVEMENT_SPEED * engine.delta_f32;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Right, KeyCode::D])
    {
        const MOVEMENT_SPEED: f32 = 100.0;
        player.translation.x += MOVEMENT_SPEED * engine.delta_f32;
    }

    // handle mouse input
    if engine.mouse_state.just_pressed(MouseButton::Left) {
        if let Some(mouse_location) = engine.mouse_state.location() {
            let label = format!("drsweatpants{}", game_state.sweatpants_index);
            game_state.sweatpants_index += 1;
            let dsp = engine.add_sprite(label.clone(), "dr_sweatpants.png");
            dsp.translation = mouse_location;
            dsp.collision = true;
            dsp.scale = 4.0;
        }
    }

    // Reset Score

    if engine.keyboard_state.just_pressed(KeyCode::R) {
        let score = engine.texts.get_mut("score").unwrap();
        game_state.score = 0;
        score.value = format!("Score {}", game_state.score);
    }
}
