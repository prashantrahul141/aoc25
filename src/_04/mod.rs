#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Item {
    Roll,
    Empty,
}

impl From<char> for Item {
    fn from(value: char) -> Self {
        match value {
            '@' => Item::Roll,
            '.' => Item::Empty,
            _ => panic!("invalid grid item"),
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    pub height: usize,
    pub width: usize,
    grid: Vec<Item>,
}

impl Grid {
    fn at(&self, y: usize, x: usize) -> Item {
        assert!(y < self.height);
        assert!(x < self.width);
        self.grid[y * self.width + x]
    }

    fn set(&mut self, y: usize, x: usize, val: Item) {
        assert!(y < self.height);
        assert!(x < self.width);
        self.grid[y * self.width + x] = val;
    }
}

fn parse(i: &'static str) -> Grid {
    let lines: Vec<&str> = i.lines().collect();
    let height = lines.len();
    let width = lines.first().unwrap().chars().count();

    let mut grid = Vec::with_capacity(height * width);
    for line in lines {
        for ch in line.chars() {
            grid.push(Item::from(ch));
        }
    }

    Grid {
        height,
        width,
        grid,
    }
}

fn total_adjacent(grid: &Grid, x: usize, y: usize) -> usize {
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && nx < grid.width as isize && ny >= 0 && ny < grid.height as isize {
                if grid.at(ny as usize, nx as usize) == Item::Roll {
                    count += 1;
                }
            }
        }
    }
    count
}

fn part_a(grid: &Grid) -> u32 {
    let mut accessible = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.at(y, x) == Item::Roll && total_adjacent(&grid, x, y) < 4 {
                accessible += 1;
            }
        }
    }
    accessible
}

fn step(old_grid: &Grid, new_grid: &mut Grid) -> usize {
    let mut removed = 0;
    for y in 0..old_grid.height {
        for x in 0..old_grid.width {
            new_grid.set(y, x, old_grid.at(y, x));
            if old_grid.at(y, x) == Item::Roll && total_adjacent(old_grid, x, y) < 4 {
                new_grid.set(y, x, Item::Empty);
                removed += 1;
            }
        }
    }
    removed
}

fn part_b(mut grid: Grid) -> usize {
    let mut total_count = 0;
    loop {
        let mut next_grid = grid.clone();
        let step_count = step(&grid, &mut next_grid);
        grid = next_grid;
        if step_count == 0 {
            break;
        }
        total_count += step_count;
    }
    total_count
}

pub fn run() {
    let i = include_str!("./input");
    let grid = parse(i);
    println!("Part A: {}", part_a(&grid));
    println!("Part B: {}", part_b(grid));
}
