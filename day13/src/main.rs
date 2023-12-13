
#[derive(Clone)]
struct Pattern {
    grid: Vec<Vec<char>>,
}

impl From<&str> for Pattern {
    fn from(s: &str) -> Self {
        Self { grid: s.lines().map(|line| line.chars().collect()).collect() }
    }
}

impl Pattern {
    fn rows_diff(&self, y1: usize, y2: usize) -> usize {
        let mut delta = 0;
        for x in 0..self.grid[0].len() {
            if self.grid[y1][x] != self.grid[y2][x] {
                delta += 1;
            }
        }
        delta
    }

    fn columns_diff(&self, x1: usize, x2: usize) -> usize {
        let mut delta = 0;
        for y in 0..self.grid.len() {
            if self.grid[y][x1] != self.grid[y][x2] {
                delta += 1;
            }
        }
        delta
    }

    fn part2(&self) -> usize {
        'rows: for i in 0..self.grid.len() - 1 {
            let mut diff = self.rows_diff(i, i + 1);
            if diff <= 1 {
                let min_distance_to_edge = i.min(self.grid.len() - i - 2);
                for d in 1..=min_distance_to_edge {
                    diff += self.rows_diff(i - d, i + d + 1);
                    if diff > 1 {
                        continue 'rows;
                    }
                }

                if diff == 0 {
                    continue 'rows;
                }
                return (i + 1) * 100;
            }
        }

        'columns: for i in 0..self.grid[0].len() - 1 {
            let mut diff = self.columns_diff(i, i + 1);
            if diff <= 1 {
                let min_distance_to_edge = i.min(self.grid[0].len() - i - 2);
                for d in 1..=min_distance_to_edge {
                    diff += self.columns_diff(i - d, i + d + 1);
                    if diff > 1 {
                        continue 'columns;
                    }
                }
                if diff == 0 {
                    continue 'columns;
                }
                return i + 1;
            }
        }

        0
    }

    fn rows_equal(&self, y1: usize, y2: usize) -> bool {
        self.grid[y1] == self.grid[y2]
    }

    fn columns_equal(&self, x1: usize, x2: usize) -> bool {
        for line in &self.grid {
            if line[x1] != line[x2] {
                return false;
            }
        }
        true
    }

    fn part1(&self) -> usize {
        'rows: for i in 0..self.grid.len() - 1 {
            if self.rows_equal(i, i + 1) {
                let min_distance_to_edge = i.min(self.grid.len() - i - 2);
                for d in 1..=min_distance_to_edge {
                    if !self.rows_equal(i - d, i + d + 1) {
                        continue 'rows;
                    }
                }

                return (i + 1) * 100;
            }
        }

        'columns: for i in 0..self.grid[0].len() - 1 {
            if self.columns_equal(i, i + 1) {
                let min_distance_to_edge = i.min(self.grid[0].len() - i - 2);
                for d in 1..=min_distance_to_edge {
                    if !self.columns_equal(i - d, i + d + 1) {
                        continue 'columns;
                    }
                }
                return i + 1;
            }
        }

        0
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let patterns = input
        .split("\n\n")
        .map(|pattern| pattern.into())
        .collect::<Vec<Pattern>>();

    let p1 = patterns.iter().map(|pattern| pattern.part1()).sum::<usize>();
    println!("Output: {}", p1); // 29846

    let p2 = patterns.iter().map(|pattern| pattern.part2()).sum::<usize>();
    println!("Output: {}", p2); // 25401
}
