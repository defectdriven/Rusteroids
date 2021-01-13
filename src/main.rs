use tcod::colors::*;
use tcod::console::*;

const SCREEN_WIDTH: i32 = 75;
const SCREEN_HEIGHT: i32 = 75;
const LIMIT_FPS: i32 = 20;

struct Tcod {
    root: Root,
}

struct Ship {
    x: i32,
    y: i32,
    vel_x: i32,
    vel_y: i32,
    angle: i32,
    radius: i32
}

impl Ship {
    pub fn update(&self) {

    }
    pub fn draw(&self) {

    }
    pub fn fire_bullet(&self) {

    }
}

fn main() {
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rusteroids")
        .init();
    let mut player_ship = Ship { x: 0, y: 0, vel_x: 0, vel_y: 0, angle: 0, radius: 5 };
    let mut tcod = Tcod { root };
    tcod::system::set_fps(LIMIT_FPS);
    while !tcod.root.window_closed() {
        tcod.root.set_default_foreground(WHITE);
        tcod.root.clear();
        tcod.root.put_char(1, 1, '@', BackgroundFlag::None);
        tcod.root.horizontal_line(20, 20, 35, BackgroundFlag::None);
        tcod.root.flush();
        // handle keys and exit game if needed
        let exit = handle_keys(&mut tcod, &mut player_ship);
        if exit {
            break;
        }
    }
}

fn render() {

}

fn handle_keys(tcod: &mut Tcod, player_ship: &mut Ship) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = tcod.root.wait_for_keypress(true);
    match key {
        Key {
            code: Enter,
            alt: true,
            ..
        } => {
            // Alt+Enter: toggle fullscreen
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => return true, // exit game

        // movement keys
        Key { code: Spacebar, .. } => player_ship.fire_bullet(),
        _ => {}
    }

    false
}