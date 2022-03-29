pub struct Grid;

impl Grid {

    pub fn print<F>(n: usize, m: usize, element_to_print: F) where F: Fn(usize, usize) -> &'static str {
        let mut y = 1;
        for _ in 0..m {
            Grid::print_line(n);
            print!("|");
            for x in 0..n {
                print!(" {} |", element_to_print(x, y));
            }
            print!("\n");
            y += 1;
        }
        Grid::print_line(n);
        print!("\n");
    }
    
    fn print_line(n: usize) {
        print!("+");
        for _ in 0..n {
            print!("---+");
        }
        print!("\n");
    }

}