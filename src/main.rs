use crate::KeyboardKey::*;
use raylib::prelude::*;
use rand::Rng;

const MOVEMENT_SPEED: f32 = 4.2;
const BALL_MAX_SPEED: f32 = 10.0;
const BALL_MIN_SPEED: f32 = 5.0;

#[derive(Default)]
struct Racket {
    position: Vector2,
    size: Vector2,
    color: Color,
}

#[derive(Default)]
struct Ball {
    position: Vector2,
    speed: Vector2,
    radius: f32,
    color: Color,
    active: bool,
}

#[derive(Default)]
struct Player {
    racket: Racket,
    score: u8,
}

struct Game {
    player_one: Player,
    player_two: Player,
    ball: Ball,
}

impl Default for Game {
    fn default() -> Self {
        let player_one = Player::default();
        let player_two = Player::default();
        let ball = Ball::default();

        Game {
            player_one,
            player_two,
            ball,
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Pong")
        .build();

    let mut game = Game::default();
    rl.set_target_fps(60);
    init(&mut game, &rl);

    while !rl.window_should_close() {
        update(&mut game, &rl);
        draw(&mut game, &mut rl, &thread);
    }
}

fn init(game: &mut Game, rl: &RaylibHandle) {
    let (w, h) = (rl.get_screen_width() as f32, rl.get_screen_height() as f32);

    // Player 1:
    game.player_one.racket.position = Vector2::new(
        rl.get_screen_width() as f32 * 0.1,
        rl.get_screen_height() as f32 / 2.0,
    );
    game.player_one.racket.size = Vector2::new(rl.get_screen_width() as f32 / 30.0, 80.0);
    game.player_one.racket.color = Color::BLUE;
    game.player_one.score = 0;

    // Player 2:
    game.player_two.racket.position = Vector2::new(
        rl.get_screen_width() as f32 * 0.9,
        rl.get_screen_height() as f32 / 2.0,
    );
    game.player_two.racket.size = Vector2::new(rl.get_screen_width() as f32 / 30.0, 80.0);
    game.player_two.racket.color = Color::RED;
    game.player_two.score = 0;

    // Ball:
    game.ball.position = Vector2::new(w / 2.0, h / 2.0);
    game.ball.speed = Vector2::default();
    game.ball.radius = 7.0;
    game.ball.color = Color::BLACK;
    game.ball.active = false;
}

fn update(game: &mut Game, rl: &RaylibHandle) {
    let (w, h) = (rl.get_screen_width() as f32, rl.get_screen_height() as f32);

    if rl.is_key_down(KEY_UP) {
        if game.player_one.racket.position.y > game.player_one.racket.size.y / 2.0 {
            game.player_one.racket.position.y -= MOVEMENT_SPEED;
        }
    }
    if rl.is_key_down(KEY_DOWN) {
        if game.player_one.racket.position.y < rl.get_screen_height() as f32 - game.player_one.racket.size.y / 2.0 {
            game.player_one.racket.position.y += MOVEMENT_SPEED;
        }
    }
    if !game.ball.active {
        if rl.is_key_pressed(KEY_SPACE) {
            let mut rng = rand::thread_rng();
            game.ball.active = true;
            while game.ball.speed.x > -BALL_MIN_SPEED && game.ball.speed.x < BALL_MIN_SPEED || game.ball.speed.y == 0.0 {
                game.ball.speed = Vector2::new(rng.gen_range(-BALL_MAX_SPEED..BALL_MAX_SPEED),
                                               rng.gen_range(-BALL_MAX_SPEED..BALL_MAX_SPEED));
            }
        }
    }

    if game.ball.active {
        game.ball.position += game.ball.speed;
        if game.ball.position.y > game.player_two.racket.position.y {
            game.player_two.racket.position.y += MOVEMENT_SPEED;
        }
        if game.ball.position.y < game.player_two.racket.position.y {
            game.player_two.racket.position.y -= MOVEMENT_SPEED;
        }
        if game.ball.speed.x > -BALL_MIN_SPEED && game.ball.speed.x < BALL_MIN_SPEED {
            if game.ball.speed.x < 0.0 {
                game.ball.speed.x = -BALL_MIN_SPEED;
            } else {
                game.ball.speed.x = BALL_MIN_SPEED;
            }
        }
        if game.ball.speed.y > -BALL_MIN_SPEED && game.ball.speed.y < BALL_MIN_SPEED {
            if game.ball.speed.y < 0.0 {
                game.ball.speed.y = -BALL_MIN_SPEED;
            } else {
                game.ball.speed.y = BALL_MIN_SPEED;
            }
        }
    } else {
        game.ball.position = Vector2::new(rl.get_screen_width() as f32 / 2.0, rl.get_screen_height() as f32 / 2.0);
    }

    if game.ball.position.y + game.ball.radius as f32 >= h || game.ball.position.y - game.ball.radius as f32 <= 0.0
    {
        game.ball.speed.y *= -1.0;
    }
    if game.ball.position.x + game.ball.radius as f32 >= w {
        game.ball.speed = Vector2::default();
        game.ball.active = false;
        game.player_one.score += 1;
    }
    if game.ball.position.x + game.ball.radius as f32 <= 0.0 {
        game.ball.speed = Vector2::default();
        game.ball.active = false;
        game.player_two.score += 1;
    }

    let r1 = Rectangle::new(
        game.player_one.racket.position.x - game.player_one.racket.size.x / 2.0,
        game.player_one.racket.position.y - game.player_one.racket.size.y / 2.0,
        game.player_one.racket.size.x,
        game.player_one.racket.size.y,
    );

    let r2 = Rectangle::new(
        game.player_two.racket.position.x - game.player_two.racket.size.x / 2.0,
        game.player_two.racket.position.y - game.player_two.racket.size.y / 2.0,
        game.player_two.racket.size.x,
        game.player_two.racket.size.y,
    );

    if r1.check_collision_circle_rec(game.ball.position, game.ball.radius as f32) {
        if game.ball.speed.x < 0.0 {
            game.ball.speed.x *= -1.0;
            game.ball.speed.y = (game.ball.position.y - game.player_one.racket.position.y)
                / (game.player_one.racket.size.y / 2.0) * BALL_MAX_SPEED;
        }
    }
    if r2.check_collision_circle_rec(game.ball.position, game.ball.radius as f32) {
        if game.ball.speed.x > 0.0 {
            game.ball.speed.x *= -1.0;
            game.ball.speed.y = (game.ball.position.y - game.player_two.racket.position.y)
                / (game.player_two.racket.size.y / 2.0) * BALL_MAX_SPEED;
        }
    }
}

fn draw(game: &Game, rl: &mut RaylibHandle, thread: &RaylibThread) {
    let (width, height) = (rl.get_screen_width() as i32, rl.get_screen_height() as i32);
    let mut d = rl.begin_drawing(thread);
    let quarter_width = width / 4;
    let quarter_height = height / 4;

    d.clear_background(Color::WHITE);

    d.draw_rectangle(
        (game.player_one.racket.position.x - game.player_one.racket.size.x / 2.0) as i32,
        (game.player_one.racket.position.y - game.player_one.racket.size.y / 2.0) as i32,
        game.player_one.racket.size.x as i32,
        game.player_one.racket.size.y as i32,
        game.player_one.racket.color,
    );

    d.draw_rectangle(
        (game.player_two.racket.position.x - game.player_two.racket.size.x / 2.0) as i32,
        (game.player_two.racket.position.y - game.player_two.racket.size.y / 2.0) as i32,
        game.player_two.racket.size.x as i32,
        game.player_two.racket.size.y as i32,
        game.player_two.racket.color,
    );

    d.draw_circle_v(game.ball.position, game.ball.radius as f32, game.ball.color);

    d.draw_text(&game.player_one.score.to_string(), quarter_width, quarter_height, 72, Color::BLACK);
    d.draw_text(&game.player_two.score.to_string(), quarter_width * 3, quarter_height, 72, Color::BLACK);

}