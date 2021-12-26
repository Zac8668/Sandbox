use macroquad::prelude::*;

mod pix;
use pix::*;
mod vec2;
use vec2::Vec2;
mod grid;
use grid::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Sandbox".to_owned(),
        window_height: 688,
        window_width: 1356,
        ..Default::default()
    }
}

const SIZE: f32 = 2.0;

#[macroquad::main(window_conf)]
async fn main() {
    let mut pixes: Vec<Pix> = Vec::new();
    let mut grid = Grid::new(
        (screen_width() / SIZE) as u32,
        (screen_height() / SIZE) as u32,
    );

    let steps = 1;

    loop {
        //input
        if is_mouse_button_pressed(MouseButton::Left) {
            let mut pos = Vec2::new(
                (mouse_position().0 / SIZE).floor() * SIZE,
                (mouse_position().1 / SIZE).floor() * SIZE,
            );
            for _ in 0..20 {
                for _ in 0..20 {
                    let in_bounds = ((pos.y / SIZE).floor() as u32) < grid.height
                        && ((pos.x / SIZE).floor() as u32) < grid.width;
                    if in_bounds && !grid.vec[(pos.y / SIZE) as usize][(pos.x / SIZE) as usize] {
                        grid.vec[(pos.y / SIZE) as usize][(pos.x / SIZE) as usize] = true;

                        let new_pix = Pix {
                            pos,
                            size: SIZE,
                            kind: 1,
                        };
                        pixes.push(new_pix);
                    }
                    pos.x += SIZE;
                }
                pos.y += SIZE;
                pos.x -= 20. * SIZE;
            }
        }

        //update
        for _ in 0..steps {
            for pix in &mut pixes {
                pix.update(&mut grid);
                draw_rectangle(pix.pos.x, pix.pos.y, pix.size, pix.size, YELLOW);
            }
        }

        //draw
        draw_text(&get_fps().to_string(), 20., 60., 40., WHITE);

        next_frame().await
    }
}
