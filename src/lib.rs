use grid::Grid;

pub mod files;
pub mod grid;
pub mod lcs;

pub struct Diff<'a> {
    grid: Grid,
    x: &'a [String],
    y: &'a [String],
}

impl<'a> Diff<'a> {
    pub fn new(x: &'a [String], y: &'a [String]) -> Self {
        Diff {
            grid: lcs::lcs(x, y),
            x,
            y,
        }
    }

    pub fn print_diff(&self) {
        print_diff(&self.grid, self.x, self.y, self.x.len(), self.y.len());
    }
}

fn print_diff(grid: &Grid, x: &[String], y: &[String], i: usize, j: usize) {
    if i > 0 && j > 0 && x[i - 1] == y[j - 1] {
        print_diff(grid, x, y, i - 1, j - 1);
    } else if j > 0 && (i == 0 || grid[[i, j - 1]] >= grid[[i - 1, j]]) {
        print_diff(grid, x, y, i, j - 1);
        println!("> {}", y[j - 1]);
    } else if i > 0 && (j == 0 || grid[[i, j - 1]] < grid[[i - 1, j]]) {
        print_diff(grid, x, y, i - 1, j);
        println!("< {}", x[i - 1]);
    } else {
        println!();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
