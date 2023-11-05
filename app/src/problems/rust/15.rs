fn main() {
    const GRID_SIZE: u16 = 40;
    let mut num: u64 = 0;
    for i in 0..GRID_SIZE/2 {
        num |= 1 << i;
    }
    let final_num = num;
    num <<= GRID_SIZE/2;
    
    let mut count = 1;

    while num != final_num {
        for i in 1..=GRID_SIZE {
            if num & (1 << i) != 0 {
                if num & (1 << (i-1)) == 0 {
                    num &= !(1 << i);
                    num |= 1 << (i - 1);
                    count+=1;
                    println!("{num:b}, {final_num:b}");
                }
            }
        }
    }
    println!("{count}");
    
}