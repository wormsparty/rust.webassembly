mod game;

use game::{Direction, Game};
use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, KeyboardEvent};
use std::rc::Rc;
use std::cell::RefCell;

#[component]
fn App() -> impl IntoView {
    let (score, set_score) = create_signal(0);
    let (game_over, set_game_over) = create_signal(false);
    
    let canvas_ref = create_node_ref::<html::Canvas>();
    let game = Rc::new(RefCell::new(Game::new(20)));
    let cell_size = 20.0;

    // Game Loop and Input handling
    let game_for_effect = Rc::clone(&game);
    create_effect(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            let ctx = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            let game_loop_ref = Rc::clone(&game_for_effect);
            
            // Keydown listener
            let keydown_game = Rc::clone(&game_for_effect);
            let handle_keydown = Closure::wrap(Box::new(move |e: KeyboardEvent| {
                match e.key().as_str() {
                    "ArrowUp" => keydown_game.borrow_mut().change_direction(Direction::Up),
                    "ArrowDown" => keydown_game.borrow_mut().change_direction(Direction::Down),
                    "ArrowLeft" => keydown_game.borrow_mut().change_direction(Direction::Left),
                    "ArrowRight" => keydown_game.borrow_mut().change_direction(Direction::Right),
                    _ => {}
                }
            }) as Box<dyn FnMut(KeyboardEvent)>);

            window().add_event_listener_with_callback("keydown", handle_keydown.as_ref().unchecked_ref()).unwrap();
            handle_keydown.forget();

            // Loop
            let mut last_tick = 0.0;
            let tick_rate = 150.0; // ms

            let f = Rc::new(RefCell::new(None));
            let g = f.clone();

            *g.borrow_mut() = Some(Closure::wrap(Box::new(move |timestamp: f64| {
                if timestamp - last_tick > tick_rate {
                    last_tick = timestamp;
                    
                    let mut g_mut = game_loop_ref.borrow_mut();
                    if !g_mut.game_over {
                        g_mut.step();
                        set_score.set(g_mut.score);
                        set_game_over.set(g_mut.game_over);
                        
                        // Render
                        render_canvas(&ctx, &g_mut, cell_size);
                    }
                }
                
                request_animation_frame(f.borrow().as_ref().unwrap());
            }) as Box<dyn FnMut(f64)>));

            request_animation_frame(g.borrow().as_ref().unwrap());
        }
    });

    let game_for_button = Rc::clone(&game);
    view! {
        <div class="ui">
            <h1>"Rust Snake"</h1>
            <div class="score">"Score: " {move || score.get()}</div>
            {move || if game_over.get() {
                view! { <div style="color: red; font-size: 20px;">"GAME OVER"</div> }.into_view()
            } else {
                view! { <div></div> }.into_view()
            }}
        </div>
        <canvas
            _ref=canvas_ref
            width="400"
            height="400"
        ></canvas>
        <div class="controls">
            "Utilisez les flèches du clavier pour diriger le serpent."
        </div>
        <button on:click=move |_| {
            let mut g_mut = game_for_button.borrow_mut();
            *g_mut = Game::new(20);
            set_score.set(0);
            set_game_over.set(false);
        }>
            "Recommencer"
        </button>
    }
}

fn render_canvas(ctx: &CanvasRenderingContext2d, game: &Game, cell_size: f64) {
    // Clear
    ctx.set_fill_style_str("#000");
    ctx.fill_rect(0.0, 0.0, 400.0, 400.0);

    // Food
    ctx.set_fill_style_str("#f44336");
    ctx.fill_rect(
        game.food.x as f64 * cell_size,
        game.food.y as f64 * cell_size,
        cell_size - 1.0,
        cell_size - 1.0,
    );

    // Snake
    for (i, pos) in game.snake.iter().enumerate() {
        if i == 0 {
            ctx.set_fill_style_str("#8bc34a");
        } else {
            ctx.set_fill_style_str("#4caf50");
        }
        ctx.fill_rect(
            pos.x as f64 * cell_size,
            pos.y as f64 * cell_size,
            cell_size - 1.0,
            cell_size - 1.0,
        );
    }
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}
