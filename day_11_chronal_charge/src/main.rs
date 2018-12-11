extern crate itertools;

use itertools::Itertools;

fn main() {
    let grid = create_fuel_grid(2866);
    println!("(x,y) of max 3 x 3 grid: {:?}", find_max_3x3_subgrid(&grid));
    println!("(x,y,n) of max n x n grid: {:?}", find_max_anysize_subgrid(&grid));
}

fn generate_summed_area_table(grid: &Vec<Vec<isize>>) -> Vec<Vec<isize>> {
    let mut sum_area = vec![vec![0; grid.len()]; grid.len()];

    for (y, values) in grid.iter().enumerate() {
        for (x, value) in values.iter().enumerate() {
            sum_area[y][x] = match (x > 0, y > 0) {
                (true, true)   => value + sum_area[y - 1][x] + sum_area[y][x - 1] - sum_area[y - 1][x - 1],
                (true, false)  => value + sum_area[y][x - 1],
                (false, true)  => value + sum_area[y - 1][x],
                (false, false) => *value
            };
        }
    }

    sum_area
}

fn sum_from_summed_area_table(sum_area: &Vec<Vec<isize>>, x: usize, y: usize, n: usize) -> isize {
    let (x0, y0) = (x.checked_sub(1).unwrap_or(0), y.checked_sub(1).unwrap_or(0));
    let (x1, y1) = (x + n - 1, y + n - 1);

    match (x > 0, y > 0) {
        (true, true)   => sum_area[y1][x1] + sum_area[y0][x0] - sum_area[y0][x1] - sum_area[y1][x0],
        (true, false)  => sum_area[y1][x1] - sum_area[y1][x0],
        (false, true)  => sum_area[y1][x1] - sum_area[y0][x1],
        (false, false) => sum_area[y1][x1]
    }
}

fn find_max_anysize_subgrid(grid: &Vec<Vec<isize>>) -> (usize, usize, usize) {
    let sum_area = generate_summed_area_table(grid);

    let (_, maxx, maxy, maxn) = (1..301)
        .fold((0, 0, 0, 0), |(maxsum, maxx, maxy, maxn), n| {
            let (x, y, sum) = find_max_subgrid(&sum_area, n, &sum_from_summed_area_table);

            match sum > maxsum {
                true => (sum, x, y, n),
                false => (maxsum, maxx, maxy, maxn)
            }
        });

    (maxx, maxy, maxn)
}

fn find_max_3x3_subgrid(grid: &Vec<Vec<isize>>) -> (usize, usize) {
    let (x, y, _) = find_max_subgrid(grid, 3, &total_power_in_subgrid);
    (x, y)
}

fn find_max_subgrid(grid: &Vec<Vec<isize>>, n: usize, sum_subgrid: &Fn(&Vec<Vec<isize>>, usize, usize, usize) -> isize) -> (usize, usize, isize) {
    let (maxsum, x, y) = (0..300 - n).cartesian_product(0..300 - n)
        .fold((0, 0, 0), |(maxsum, maxx, maxy), (x, y)| {
            let sum = sum_subgrid(&grid, x, y, n);
            match sum > maxsum {
                true => (sum, x, y),
                false => (maxsum, maxx, maxy)
            }
        });

    (x + 1, y + 1, maxsum)
}

fn total_power_in_subgrid(grid: &Vec<Vec<isize>>, x: usize, y: usize, n: usize) -> isize {
    let x = x;
    let y = y;

    (x..x + n).cartesian_product(y..y + n)
        .fold(0, |sum, (x, y)| sum + grid[y][x])
}

fn create_fuel_grid(offset: isize) -> Vec<Vec<isize>> {
    let grid = (0..300).cartesian_product(0..300)
        .fold(vec![vec![0; 300]; 300], |mut grid, (x, y)| {
            grid[y][x] = power_level(1 + x as isize, 1 + y as isize, offset);
            grid
        });

    grid
}

fn power_level(x: isize, y: isize, offset: isize) -> isize {
    (((offset*x + x.pow(2)*y + 20*x*y + 100*y + 10*offset) / 100) % 10) - 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_level() {
        assert_eq!(power_level(3, 5, 8), 4);
        assert_eq!(power_level(122, 79, 57), -5);
        assert_eq!(power_level(217, 196, 39), 0);
        assert_eq!(power_level(101, 153, 71), 4);
    }

    #[test]
    fn test_find_max_3x3_subgrid() {
        let grid18 = create_fuel_grid(18);
        assert_eq!(find_max_3x3_subgrid(&grid18), (33,45));

        let grid42 = create_fuel_grid(42);
        assert_eq!(find_max_3x3_subgrid(&grid42), (21,61));
    }
}
