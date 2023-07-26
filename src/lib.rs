use rand::Rng;
use std::fs;

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
            "gosper_glider_gun" => {
                //grid.height = 30;
                //grid.width = 30;
                grid.generate_grid("gosper_glider_gun.txt");
            }
            // TODO [NH] fix the shape generation
            _ => grid.generate_random_grid(probability),
        }
        grid
    }

    pub fn generate_grid(&mut self, file_path: &str) {
        let binding =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        let lines = binding.lines().collect::<Vec<&str>>();

        for (i, line) in lines.iter().enumerate() {
            for (j, char) in line.chars().enumerate() {
                self.data[i][j] = char == 'O';
            }
        }
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
