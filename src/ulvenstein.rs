use serde::{Deserialize, Serialize};
use std::sync::RwLock;
use std::cmp::{max, min};

const MAP_WIDTH: usize = 24;
const MAP_HEIGHT: usize = 24;

const SCREEN_WIDTH: usize = 640;
const SCREEN_HEIGHT: usize = 480;

const ROTATION_SPEED: f32 = 0.08;
const MOVEMENT_SPEED: f32 = 0.1;

static STATE: RwLock<State> = RwLock::new(State::new_const_default());
pub static INPUT: RwLock<Vec<Keys>> = RwLock::new(vec![]);

static WORLD: [[u8; MAP_WIDTH]; MAP_HEIGHT] = [
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
];

#[derive(Serialize, Deserialize)]
pub struct Keys {
    pub left: bool,
    pub up: bool,
    pub right: bool,
    pub down: bool,
}

impl Keys {
    const fn new_const_default() -> Self {
        Self {
            left: false,
            up: false,
            right: false,
            down: false,
        }
    }
}

impl Default for Keys {
    fn default() -> Self {
        Self::new_const_default()
    }
}

pub struct State {
    // Positional x,y
    pub pos_x: f32,
    pub pos_y: f32,

    // Camera orientation
    pub dir_x: f32,
    pub dir_y: f32,
    pub plane_x: f32,
    pub plane_y: f32,
}

impl State {
    const fn new_const_default() -> Self {
        Self {
            pos_x: 22.0,
            pos_y: 12.0,
            dir_x: -1.0,
            dir_y: 0.0,
            plane_x: 0.0,
            plane_y: 0.66,
        }
    }
}
impl Default for State {
    fn default() -> Self {
        Self::new_const_default()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Stroke {
    pub x: i32,
    pub y_start: i32,
    pub y_end: i32,
    pub color: i32,
    pub dark_hue: i32,
}

impl Stroke {
    pub fn new(x: i32, y_start: i32, y_end: i32, color: i32, dark_hue: i32) -> Stroke {
        Stroke {
            x,
            y_start,
            y_end,
            color,
            dark_hue,
        }
    }
}

fn move_left() {
    if let Ok(mut state) = STATE.write() {
        let old_dir_x = state.dir_x;
        state.dir_x = state.dir_x * ROTATION_SPEED.cos() - state.dir_y * ROTATION_SPEED.sin();
        state.dir_y = old_dir_x * ROTATION_SPEED.sin() + state.dir_y * ROTATION_SPEED.cos();

        let old_plane_x = state.plane_x;
        state.plane_x = state.plane_x * ROTATION_SPEED.cos() - state.plane_y * ROTATION_SPEED.sin();
        state.plane_y = old_plane_x * ROTATION_SPEED.sin() + state.plane_y * ROTATION_SPEED.cos();
    }
}

fn move_right() {
    if let Ok(mut state) = STATE.write() {
        let old_dir_x = state.dir_x;
        state.dir_x = state.dir_x * ROTATION_SPEED.cos() + state.dir_y * ROTATION_SPEED.sin();
        state.dir_y = old_dir_x * -ROTATION_SPEED.sin() + state.dir_y * ROTATION_SPEED.cos();

        let old_plane_x = state.plane_x;
        state.plane_x = state.plane_x * ROTATION_SPEED.cos() + state.plane_y * ROTATION_SPEED.sin();
        state.plane_y = old_plane_x * -ROTATION_SPEED.sin() + state.plane_y * ROTATION_SPEED.cos();
    }
}

fn move_forward() {
    if let Ok(mut state) = STATE.write() {
        let map_x = (state.pos_x + state.dir_x * MOVEMENT_SPEED).trunc() as usize;
        let map_y = (state.pos_y + state.dir_y * MOVEMENT_SPEED).trunc() as usize;

        if WORLD[map_x][state.pos_y.trunc() as usize] == 0 {
            state.pos_x += state.dir_x * MOVEMENT_SPEED;
        }
        if WORLD[state.pos_x.trunc() as usize][map_y] == 0 {
            state.pos_y += state.dir_y * MOVEMENT_SPEED;
        }
    }
}

fn move_backward() {
    if let Ok(mut state) = STATE.write() {
        let map_x = (state.pos_x - state.dir_x * MOVEMENT_SPEED).trunc() as usize;
        let map_y = (state.pos_y - state.dir_y * MOVEMENT_SPEED).trunc() as usize;

        if WORLD[map_x][state.pos_y.trunc() as usize] == 0 {
            state.pos_x -= state.dir_x * MOVEMENT_SPEED;
        }

        if WORLD[state.pos_x.trunc() as usize][map_y] == 0 {
            state.pos_y -= state.dir_y * MOVEMENT_SPEED;
        }
    }
}

pub fn game_loop() -> Vec<Stroke> {
    // Redraw the frame if there is input to handle as
    // the camera has potentially changed.
    if handle_input() {
        return render_frame();
    }

    vec![]
}

fn handle_input() -> bool {
    if let Ok(mut keys) = INPUT.write() {
        if keys.is_empty() {
            return false;
        }

        for key in keys.drain(0..).collect::<Vec<Keys>>() {
            if key.left {
                move_left();
            }
            if key.up {
                move_forward();
            }
            if key.right {
                move_right();
            }
            if key.down {
                move_backward();
            }
        }

        return true;
    };

    false
}

fn render_frame() -> Vec<Stroke> {
    let mut screen: Vec<Stroke> = vec![];

    if let Ok(state) = STATE.read() {
        for x in 0..SCREEN_WIDTH {
            let camera_x: f32 = 2.0 * x as f32 / SCREEN_WIDTH as f32 - 1.0;
            let ray_dir_x: f32 = state.dir_x + state.plane_x * camera_x;
            let ray_dir_y: f32 = state.dir_y + state.plane_y * camera_x;

            let delta_dist_x: f32 = if ray_dir_x == 0.0 {
                f32::INFINITY
            } else {
                (1.0 / ray_dir_x).abs()
            };
            let delta_dist_y: f32 = if ray_dir_y == 0.0 {
                f32::INFINITY
            } else {
                (1.0 / ray_dir_y).abs()
            };

            let mut map_x: usize = state.pos_x.trunc() as usize;
            let mut map_y: usize = state.pos_y.trunc() as usize;

            let step_x = if ray_dir_x < 0.0 { -1 } else { 1 };
            let step_y = if ray_dir_y < 0.0 { -1 } else { 1 };

            let mut side_dist_x = if ray_dir_x < 0.0 {
                (state.pos_x - map_x as f32) * delta_dist_x
            } else {
                (map_x as f32 + 1.0 - state.pos_x) * delta_dist_x
            };
            let mut side_dist_y = if ray_dir_y < 0.0 {
                (state.pos_y - map_y as f32) * delta_dist_y
            } else {
                (map_y as f32 + 1.0 - state.pos_y) * delta_dist_y
            };

            // perform dda
            let mut hit: i32 = 0;
            let mut side: i32 = 0;
            while hit == 0 {
                if side_dist_x < side_dist_y {
                    side_dist_x += delta_dist_x;
                    map_x = (map_x as i32 + step_x) as usize;
                    side = 0;
                } else {
                    side_dist_y += delta_dist_y;
                    map_y = (map_y as i32 + step_y) as usize;
                    side = 1;
                }

                if WORLD[map_x][map_y] > 0 {
                    hit = 1;
                }
            }

            let perp_wall_dist = if side == 0 {
                side_dist_x - delta_dist_x
            } else {
                side_dist_y - delta_dist_y
            };

            let height: isize = (SCREEN_HEIGHT as f32 / perp_wall_dist) as isize;
            let y_start: isize = max(-height / 2 + SCREEN_HEIGHT as isize / 2, 0);
            let y_end = min(height / 2 + SCREEN_HEIGHT as isize / 2, SCREEN_HEIGHT as isize - 1);
            let color: u8 = WORLD[map_x][map_y];
            let dark_hue: bool = side == 1;

            screen.push(Stroke::new(
                x as i32,
                y_start as i32,
                y_end as i32,
                color as i32,
                dark_hue as i32,
            ));
        }
    }

    screen
}
