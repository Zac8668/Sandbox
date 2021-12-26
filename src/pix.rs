use crate::rand::gen_range;
use crate::vec2::Vec2;
use crate::Grid;

#[derive(Clone)]
pub struct Pix {
    pub pos: Vec2,
    pub size: f32,
    pub kind: u8,
}

impl Pix {
    pub fn update(&mut self, grid: &mut Grid) {
        let x = (self.pos.x / self.size) as usize;
        let y = (self.pos.y / self.size) as usize;

        let y_bounds = ((y + 1) as u32) < grid.height;
        let l_bounds = (x as i32 - 1) >= 0;
        let r_bounds = ((x + 1) as u32) < grid.width;

        if y_bounds && !grid.vec[y + 1][x] {
            self.get_down(1, grid);
        } else if y_bounds && grid.vec[y + 1][x] {
            let free_left = l_bounds && !(grid.vec[y + 1][x - 1]);
            let free_right = r_bounds && !(grid.vec[y + 1][x + 1]);

            if free_left && free_right {
                self.get_down(gen_range(0, 2) * 2, grid);
            } else if free_left {
                self.get_down(0, grid);
            } else if free_right {
                self.get_down(2, grid);
            }
        }
    }

    pub fn get_down(&mut self, pos: u8, grid: &mut Grid) {
        let x = (self.pos.x / self.size) as usize;
        let y = (self.pos.y / self.size) as usize;

        if pos == 0 {
            grid.vec[y][x] = false;
            self.pos.y += self.size;
            self.pos.x -= self.size;
            grid.vec[y + 1][x - 1] = true;
        } else if pos == 1 {
            grid.vec[y][x] = false;
            self.pos.y += self.size;
            grid.vec[y + 1][x] = true;
        } else if pos == 2 {
            grid.vec[y][x] = false;
            self.pos.y += self.size;
            self.pos.x += self.size;
            grid.vec[y + 1][x + 1] = true;
        }
    }
}
