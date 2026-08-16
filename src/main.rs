/*use std::cmp::Ordering;
use std::io;

use rand::Rng;

/// This a little program that lets the user guess a secret random number between 1 and 100 (inclusive)
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
*/

/*
use macroquad::prelude::*;

fn conf() -> Conf {
    Conf {
        window_title: "Psygon!".to_string(),
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let (x, y) = (screen_width() / 2., screen_height() / 2.);
    let r: f32 = 70.;
    let circle = Circle::new(x, y, r);
    let mut score = 0;

    loop {
        clear_background(GRAY);

        draw_text("Clicker Game", screen_width() / 2. - 100., 100., 50., WHITE);
        draw_text(
            format!("Clicks: {}", score).as_str(),
            screen_width() / 2. - 100.,
            500.,
            50.,
            WHITE,
        );

        draw_circle(x, y, r, RED);

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();

            if circle.contains(&Vec2::new(mouse_x, mouse_y)) {
                score += 1;
            }
        }

        next_frame().await;
    }
}
*/

use macroquad::prelude::*;

enum Location {
    Menu,
    Game,
}

fn conf() -> Conf {
    Conf {
        window_title: "Psygon!".to_string(),
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    set_pc_assets_folder("../assets");

    let mut location = Location::Menu;
    let texture = load_texture("background.png").await.unwrap();
    let mut test: i32 = 0;

    loop {
        clear_background(LIGHTGRAY);

        match location {
            Location::Menu => draw_texture(&texture, 0.0, 0.0, WHITE),
            Location::Game => (),
        }

        if test == 0 {
            location = Location::Game;
            test += 1;
        }

        next_frame().await;
    }
}
