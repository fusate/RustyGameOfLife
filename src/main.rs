extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use rand::{Rng, thread_rng};

use std::time::Duration;

// dimensions of the grid
const HEIGHT:usize = 50;
const WIDTH:usize = 100;

// how many pixels to use per cell when rendering(cells are square always)
const SCALE:u8 = 8;

/** randomly generate  */
fn gen_world() -> [[bool; HEIGHT]; WIDTH]{
    let mut world = [[false; HEIGHT];WIDTH];
    let mut rng = thread_rng();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            world[x][y] = rng.gen_bool(0.5);
        }
    }
    return world;
}

fn get_neighbour_num(world: [[bool; HEIGHT]; WIDTH], x: usize, y: usize) -> u8{
    let mut num_neighbours:u8 = 0;
    // i corresponds to the x offset, j corresponds to the y offset
    for i in -1 as i8 ..=1 {
        for j in -1 as i8..=1 {

            if i == 0 && j == 0 { // don't count the cell itself
                continue;
            } 

            let check_x = usize::try_from(x as i64+i as i64);
            let check_y = usize::try_from(y as i64+j as i64);

            if check_x.is_ok() && check_y.is_ok() && check_x.unwrap() < WIDTH && check_y.unwrap() < HEIGHT{
                if world[check_x.unwrap()][check_y.unwrap()] {
                    num_neighbours += 1;
                }
            }
        }
    }
    return num_neighbours;
}

fn step(inp: [[bool; HEIGHT]; WIDTH], out: &mut [[bool; HEIGHT]; WIDTH]) {
    let mut neighbour_num: u8;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            neighbour_num = get_neighbour_num(inp, x, y);
            if inp[x as usize][y as usize] {// if the cell is live
                // if the live cell has 2 or 3 neighbours
                if neighbour_num == 2 || neighbour_num == 3 {
                    out[x][y] = true;
                } else {
                    out[x][y] = false;
                }
            } else {//if the cell is dead
                if neighbour_num == 3 {
                    out[x][y] = true;
                } else {
                    out[x][y] = false;
                }
            }
        }
    }
}

fn render(grid: [[bool; HEIGHT]; WIDTH], canvas: &mut Canvas<Window>) {
    // background
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    // start making rectangles
    canvas.set_draw_color(Color::RGB(255, 0, 0));

    let mut width_offset: u32 = 0;
    let mut height_offset: u32 = 0;

    for x in grid {
        for y in x {
            if y {
                canvas.fill_rect(Rect::new(width_offset as i32, height_offset as i32, SCALE as u32, SCALE as u32));
            }
            height_offset += SCALE as u32;
        }
        height_offset = 0;
        width_offset += SCALE as u32;
    }
    canvas.present();
}

fn main() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Conway's Game of Life", WIDTH as u32 * SCALE as u32, HEIGHT as u32 * SCALE as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();


    println!("Welcome to Conway's Game of Life!");

    // generate the world
    // index as world[x][y]
    let mut buf = [[[false; HEIGHT]; WIDTH]; 2];
    buf[0] = gen_world();
    buf[1] = [[false; HEIGHT]; WIDTH];
    let mut current:bool = true;
    loop{
        
        if current {
            render(buf[0], &mut canvas);
            step(buf[0], &mut buf[1]);
        } else {
            render(buf[1], &mut canvas);
            step(buf[1], &mut buf[0]);
        }

        current = !current;
        ::std::thread::sleep(Duration::new(1, 0));
    }
}
