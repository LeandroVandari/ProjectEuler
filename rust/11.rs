use std::fs;
fn main() {
    let grid: Vec<Vec<u32>> = fs::read_to_string("11_grid").expect("You forgot the file dumbass").split('\n').map(|line| line.split_ascii_whitespace().map(|num| num.parse::<u32>().unwrap()).collect::<Vec<u32>>()).filter(|ls| ls.len() != 0).collect();


    let rows = grid.len();
    let columns = grid[0].len();
    let mut biggest_mult = 0;

    for irow in 0..rows {
        for inumber in 0..columns {
            if inumber + 3 < columns {
                let mut right = 1;
                for i in 0..4 {
                    right *= grid[irow][inumber + i];
                }
                if right > biggest_mult {biggest_mult = right;}
            }
            if irow + 3 < rows {
                let mut down = 1;
                for i in 0..4 {
                    down *= grid[irow + i][inumber];
                }
                if down > biggest_mult {biggest_mult = down;}
            }
            if inumber + 3 < columns && irow + 3 < rows {
                let mut down_right = 1;
                for i in 0..4 {
                    down_right *= grid[irow + i][inumber + i];
                }
                if down_right > biggest_mult {biggest_mult = down_right;}
            }
            if inumber >= 3 && irow + 3 < rows {
                let mut down_left = 1; 
                for i in 0..4 {
                    down_left *= grid[irow + i][inumber - i];
                }
                if down_left > biggest_mult {biggest_mult = down_left;}
            }
        }

    }

    println!("{biggest_mult}");

    
}
