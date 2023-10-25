use rand::Rng;
use raylib::prelude::*;

const GRID_SIZE: usize = 20;
const WINDOW_SIZE: usize = 800;
static PIXEL_SIZE: f32 = WINDOW_SIZE as f32 / GRID_SIZE as f32;

#[derive(Debug, Clone, Copy, PartialEq)]
enum CellState {
    Hidden,
    Flagged,
    Revealed,
}

type Grid = Vec<Vec<(i32, CellState)>>;

fn reveal(state: &mut Grid, position: (usize, usize)) {

    if state[position.0][position.1].0 == 0 && state[position.0][position.1].1 == CellState::Hidden {
        state[position.0][position.1].1 = CellState::Revealed;
        for x in -1..=1i32 {
            for y in -1..=1i32 {
                if x.abs() == y.abs() {
                    continue;
                }
                if (x == 0 && y == 0) {
                    continue;
                }
                let x = position.0 as i32 + x;
                let y = position.1 as i32 + y;
                if (x >= 0 && x < GRID_SIZE as i32 && y >= 0 && y < GRID_SIZE as i32) && state[x as usize][y as usize].1 == CellState::Hidden {
                    reveal(state, (x as usize, y as usize));
                }
            }
        }
    }

    if state[position.0][position.1].1 == CellState::Hidden && state[position.0][position.1].0 != -1 {
        state[position.0][position.1].1 = CellState::Revealed;
    }

}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_SIZE as i32, WINDOW_SIZE as i32)
        .title("MineSweeper")
        //.transparent()
        //.undecorated()
        .build();

    rl.set_target_fps(60);

    let mut grid : Grid = vec![vec![(0, CellState::Hidden); GRID_SIZE]; GRID_SIZE];

    // We populate random mines with -1
    let mut rng = rand::thread_rng();
    for _ in 0..20 {
        let x = rng.gen_range(0..GRID_SIZE);
        let y = rng.gen_range(0..GRID_SIZE);
        grid[x][y].0 = -1;
    }

    // We now calculate the number of mines around each cell
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            if grid[x][y].0 == -1 {
                continue;
            }

            let mut count = 0;
            for i in -1..=1 {
                for j in -1..=1 {
                    if x as i32 + i < 0 || x as i32 + i >= GRID_SIZE as i32 {
                        continue;
                    }
                    if y as i32 + j < 0 || y as i32 + j >= GRID_SIZE as i32 {
                        continue;
                    }
                    if grid[(x as i32 + i) as usize][(y as i32 + j) as usize].0 == -1 {
                        count += 1;
                    }
                }
            }
            grid[x][y] = (count, CellState::Hidden);
        }
    }

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);


        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                let color = match grid[x][y].0 {
                    -1 => Color::RED,
                    0 => Color::WHITE,
                    1 => Color::BLUE,
                    2 => Color::GREEN,
                    3 => Color::PURPLE,
                    4 => Color::ORANGE,
                    5 => Color::YELLOW,
                    6 => Color::PINK,
                    7 => Color::BROWN,
                    8 => Color::GRAY,
                    _ => Color::BLACK,
                };
                match grid[x][y].1 {
                    CellState::Hidden => d.draw_rectangle(x as i32 * PIXEL_SIZE as i32, y as i32 * PIXEL_SIZE as i32, PIXEL_SIZE as i32, PIXEL_SIZE as i32, Color::GRAY),
                    CellState::Flagged => d.draw_rectangle(x as i32 * PIXEL_SIZE as i32, y as i32 * PIXEL_SIZE as i32, PIXEL_SIZE as i32, PIXEL_SIZE as i32, Color::RED),
                    CellState::Revealed => {
                        d.draw_rectangle(x as i32 * PIXEL_SIZE as i32, y as i32 * PIXEL_SIZE as i32, PIXEL_SIZE as i32, PIXEL_SIZE as i32, Color::DARKGRAY);
                        d.draw_text(&grid[x][y].0.to_string(), x as i32 * PIXEL_SIZE as i32 + PIXEL_SIZE as i32 / 2, y as i32 * PIXEL_SIZE as i32 + PIXEL_SIZE as i32 / 2, 20, color);
                    }
                    
                }
            }
        }

        if d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {

            let x = d.get_mouse_x() as usize / (WINDOW_SIZE / GRID_SIZE);
            let y = d.get_mouse_y() as usize / (WINDOW_SIZE / GRID_SIZE);

            if grid[x][y].0 == -1 {
                println!("You lost!");
                break;
            }

            reveal(&mut grid, (x, y));

        }

        if d.is_mouse_button_pressed(MouseButton::MOUSE_RIGHT_BUTTON) {
            let x = d.get_mouse_x() as usize / (WINDOW_SIZE / GRID_SIZE);
            let y = d.get_mouse_y() as usize / (WINDOW_SIZE / GRID_SIZE);
            if grid[x][y].1 == CellState::Hidden {
                grid[x][y].1 = CellState::Flagged;
            } else if grid[x][y].1 == CellState::Flagged {
                grid[x][y].1 = CellState::Hidden;
            }
        }


    }
}
