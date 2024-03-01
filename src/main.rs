use tetra::ContextBuilder;
use tetra::State;

struct GameState{}

impl State for GameState {}

fn main() -> tetra::Result {
    ContextBuilder::new("lander-game", 1280, 720)
        .quit_on_escape(true)
        .fullscreen(true)
        .build()?
        .run(|_|Ok(GameState {}))
}

