use macroquad::prelude::*;

use std::{thread};
use std::sync::{Mutex, Arc};

fn window_conf() -> Conf {
    Conf {
        window_title: "Colorful Game of Life".to_string(),
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {

    let prob_range = 20;
    let scale =2;

    let w = (screen_width()/(scale as f32)) as usize;
    let h = (screen_height()/(scale as f32)) as usize;

    let mut image = Image::gen_image_color(w as u16, h as u16,Color::new(0.0,0.0,0.0,0.0));

    let texture = Texture2D::from_image(&image);

    let mut temp_r:Vec<u8> = vec![0;w*h];
    let mut temp_g:Vec<u8>  = vec![0;w*h];
    let mut temp_b:Vec<u8>  = vec![0;w*h];

    for i in 0..=(w*h)-1 {
        if rand::gen_range(0, prob_range) == 0 {
            temp_r[i] = 255;
        }
        if rand::gen_range(0, prob_range) == 0 {
            temp_g[i] = 255;
        }
        if rand::gen_range(0, prob_range) == 0 {
            temp_b[i] = 255;
        }
    }

    let cells_r = Arc::new(Mutex::new(temp_r.clone()));
    let buffer_r = Arc::new(Mutex::new(temp_r.clone()));

    let cells_g = Arc::new(Mutex::new(temp_g.clone()));
    let buffer_g = Arc::new(Mutex::new(temp_g.clone()));

    let cells_b = Arc::new(Mutex::new(temp_b.clone()));
    let buffer_b = Arc::new(Mutex::new(temp_b.clone()));

    //drop works, because Vector doesn't implement 'Copy' trait
    drop(temp_r);
    drop(temp_g);
    drop(temp_b);



    loop {

        let mut handles = vec![];
        // println!("FPS: {}",time::get_fps());

        let cells_r_arc = Arc::clone(&cells_r);
        let buffer_r_arc = Arc::clone(&buffer_r);
        let cells_g_arc = Arc::clone(&cells_g);
        let buffer_g_arc = Arc::clone(&buffer_g);
        let cells_b_arc = Arc::clone(&cells_b);
        let buffer_b_arc = Arc::clone(&buffer_b);

        let handle = thread::spawn(move || {

            let cells_r_access=cells_r_arc.lock().unwrap();
            let mut buffer_r_access=buffer_r_arc.lock().unwrap();

            for y in 0..h as i32 {
                for x in 0..w as i32 {
                    let mut neighbour_count = 0;

                    for j in -1i32..=1 {
                        for i in -1i32..=1 {
                            // out of bounds
                            if y + j < 0 || y + j >= h as i32 || x + i < 0 || x + i >= w as i32 {
                                continue;
                            }
                            // cell itself
                            if i == 0 && j == 0 {
                                continue;
                            }

                            let neighbor = cells_r_access[(y + j) as usize * w + (x + i) as usize];
                            if neighbor == 255 {
                                neighbour_count += 1;
                            }
                        }
                    }

                    let current_cell = cells_r_access[y as usize * w + x as usize];
                    buffer_r_access[y as usize * w + x as usize] = match (current_cell, neighbour_count) {
                        // Rule 1: Any live cell with fewer than two live neighbours
                        // dies, as if caused by underpopulation.
                        (255, x) if x < 2 => 0,
                        // Rule 2: Any live cell with two or three live neighbours
                        // lives on to the next generation.
                        (255, 2) | (255, 3) => 255,
                        // Rule 3: Any live cell with more than three live
                        // neighbours dies, as if by overpopulation.
                        (255, x) if x > 3 => 0,
                        // Rule 4: Any dead cell with exactly three live neighbours
                        // becomes a live cell, as if by reproduction.
                        (0, 3) => 255,
                        // All other cells remain in the same state.
                        (otherwise, _) => otherwise,
                    };
                }
            }





        });
        handles.push(handle);


        let handle = thread::spawn(move || {

            let cells_g_access=cells_g_arc.lock().unwrap();
            let mut buffer_g_access=buffer_g_arc.lock().unwrap();

            for y in 0..h as i32 {
                for x in 0..w as i32 {
                    let mut neighbour_count = 0;

                    for j in -1i32..=1 {
                        for i in -1i32..=1 {
                            // out of bounds
                            if y + j < 0 || y + j >= h as i32 || x + i < 0 || x + i >= w as i32 {
                                continue;
                            }
                            // cell itself
                            if i == 0 && j == 0 {
                                continue;
                            }

                            let neighbor = cells_g_access[(y + j) as usize * w + (x + i) as usize];
                            if neighbor == 255 {
                                neighbour_count += 1;
                            }
                        }
                    }

                    let current_cell = cells_g_access[y as usize * w + x as usize];
                    buffer_g_access[y as usize * w + x as usize] = match (current_cell, neighbour_count) {
                        // Rule 1: Any live cell with fewer than two live neighbours
                        // dies, as if caused by underpopulation.
                        (255, x) if x < 2 => 0,
                        // Rule 2: Any live cell with two or three live neighbours
                        // lives on to the next generation.
                        (255, 2) | (255, 3) => 255,
                        // Rule 3: Any live cell with more than three live
                        // neighbours dies, as if by overpopulation.
                        (255, x) if x > 3 => 0,
                        // Rule 4: Any dead cell with exactly three live neighbours
                        // becomes a live cell, as if by reproduction.
                        (0, 3) => 255,
                        // All other cells remain in the same state.
                        (otherwise, _) => otherwise,
                    };
                }
            }

        });
        handles.push(handle);


        let handle = thread::spawn(move || {

            let cells_b_access=cells_b_arc.lock().unwrap();
            let mut buffer_b_access=buffer_b_arc.lock().unwrap();

            for y in 0..h as i32 {
                for x in 0..w as i32 {
                    let mut neighbour_count = 0;

                    for j in -1i32..=1 {
                        for i in -1i32..=1 {
                            // out of bounds
                            if y + j < 0 || y + j >= h as i32 || x + i < 0 || x + i >= w as i32 {
                                continue;
                            }
                            // cell itself
                            if i == 0 && j == 0 {
                                continue;
                            }

                            let neighbor = cells_b_access[(y + j) as usize * w + (x + i) as usize];
                            if neighbor == 255 {
                                neighbour_count += 1;
                            }
                        }
                    }

                    let current_cell = cells_b_access[y as usize * w + x as usize];
                    buffer_b_access[y as usize * w + x as usize] = match (current_cell, neighbour_count) {
                        // Rule 1: Any live cell with fewer than two live neighbours
                        // dies, as if caused by underpopulation.
                        (255, x) if x < 2 => 0,
                        // Rule 2: Any live cell with two or three live neighbours
                        // lives on to the next generation.
                        (255, 2) | (255, 3) => 255,
                        // Rule 3: Any live cell with more than three live
                        // neighbours dies, as if by overpopulation.
                        (255, x) if x > 3 => 0,
                        // Rule 4: Any dead cell with exactly three live neighbours
                        // becomes a live cell, as if by reproduction.
                        (0, 3) => 255,
                        // All other cells remain in the same state.
                        (otherwise, _) => otherwise,
                    };
                }
            }
        });
        handles.push(handle);

        for future  in handles{
            future.join().unwrap();
        }

        let mut cells_r = cells_r.lock().unwrap();
        let mut cells_g = cells_g.lock().unwrap();
        let mut cells_b = cells_b.lock().unwrap();

        let buffer_r = buffer_r.lock().unwrap();
        let buffer_g = buffer_g.lock().unwrap();
        let buffer_b = buffer_b.lock().unwrap();

        for i in 0..=(w*h - 1) {
            cells_r[i] = buffer_r[i];
            cells_g[i] = buffer_g[i];
            cells_b[i] = buffer_b[i];
            image.set_pixel(
                (i % w) as u32,
                (i / w) as u32,
                Color::new(
                    cells_r[i] as f32,
                    cells_g[i] as f32,
                    cells_b[i] as f32,
                    100.0,
                ),
            );
        }

        texture.update(&image);

        draw_texture_ex(texture, 0., 0.,
                        Color::new(
                            10.0,
                            10.0,
                            10.0,
                            1.0,
                        ),
                        DrawTextureParams {
                            dest_size: Some(Vec2::new((w*scale) as f32,(h*scale) as f32)),
                            source: Some(Rect::new(0.0, 0.0, w as f32, h as f32 ) ),
                            ..Default::default()
                        }
        );
        next_frame().await;
    }
}