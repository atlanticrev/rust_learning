pub fn print_tree(lvl: u8) {
    for i in 1..=lvl {
        // Prefix spaces
        for _j in 1..=(lvl - i) {
            print!(" ");
        }
        // Symbols
        for j in 1..=i {
            if j % 3 == 0 {
                print!("{}", "0 ");
            } else {
                print!("{}", "* ");
            }
        }
        // Suffix spaces
        for _j in 1..=(lvl - i) {
            print!(" ");
        }
        println!();
    }
}