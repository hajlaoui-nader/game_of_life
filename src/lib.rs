use rand::Rng;

pub const WIDTH: usize = 140;
pub const HEIGHT: usize = 120;

pub struct Grid<'a> {
    width: usize,
    height: usize,
    data: Vec<Vec<bool>>,
    shape: &'a str,
}

impl Grid<'_> {
    pub fn new(width: usize, height: usize, probability: f64, shape: &str) -> Grid {
        let mut grid = Grid {
            width,
            height,
            data: vec![vec![false; width]; height],
            shape,
        };

        match shape {
            "random" => { 
                println!("random");
                grid.generate_random_grid(probability)
            },
            // TODO [NH] fix the shape generation
            _ => grid.generate_random_grid(probability),
        }
        grid
    }

    pub fn generate_random_grid(&mut self, probability: f64) {
        let mut rng = rand::thread_rng();

        for y in 0..self.height {
            for x in 0..self.width {
                self.data[y][x] = rng.gen_bool(probability); // Adjust the probability as desired
            }
        }
    }

    pub fn print_grid(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = if self.data[y][x] { '█' } else { ' ' };
                print!("{}", cell);
            }
            println!();
        }
    }

    pub fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;

        for dy in (y as i32 - 1)..=(y as i32 + 1) {
            for dx in (x as i32 - 1)..=(x as i32 + 1) {
                if dx >= 0
                    && dy >= 0
                    && (dx as usize) < self.width
                    && (dy as usize) < self.height
                    && (dx != x as i32 || dy != y as i32)
                {
                    if self.data[dy as usize][dx as usize] {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    pub fn update_grid(&mut self) {
        let mut new_data = self.data.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let neighbors = self.count_neighbors(x, y);

                new_data[y][x] = if self.data[y][x] {
                    neighbors == 2 || neighbors == 3
                } else {
                    neighbors == 3
                };
            }
        }

        self.data = new_data;
    }
}
