extern crate itertools;

use itertools::Itertools;

fn main() {
    let grid = create_fuel_grid(2866);
    println!("(x,y) of max 3 x 3 grid: {:?}", find_max_3x3_subgrid(&grid));
 
    println!("(x,y,n) of max n x n grid: {:?}", find_max_anysize_subgrid(&grid));
}

fn find_max_anysize_subgrid(grid: &Vec<Vec<isize>>) -> (usize, usize, usize) {
    let (mut maxsum, mut maxx, mut maxy, mut maxn) = (0, 0, 0, 0);

    // bruteforce way...
    for n in 1..301 {
        let (x, y, sum) = find_max_subgrid(grid, n);
        println!("({}, {}, {}): {}", x, y, n, sum);

        if sum == 0 { break }

        match sum > maxsum {
            true => {
                maxsum = sum;
                maxx = x;
                maxy = y;
                maxn = n;},
            _ => {}
        };
    }

    (maxx + 1, maxy + 1, maxn)
}

fn find_max_3x3_subgrid(grid: &Vec<Vec<isize>>) -> (usize, usize) {
    let (x, y, _) = find_max_subgrid(grid, 3);
    (x, y)
}

fn find_max_subgrid(grid: &Vec<Vec<isize>>, n: usize) -> (usize, usize, isize) {
    let (maxsum, x, y) = (0..300 - n).cartesian_product(0..300 - n)
        .fold((0, 0, 0), |(maxsum, maxx, maxy), (x, y)| {
            let sum = total_power_in_subgrid(&grid, x, y, n);
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
