/**
 * @author Jack Robbins
 * 
 * In the festive spirit, this project simply prints an ASCII christmas tree to 
 * the terminal 
*/


fn main() {
    println!("\n                    Merry Christmas!!\n");
    christmas_tree();
    println!();
}

fn christmas_tree(){
    for n in (0..27).rev(){
        println!("{}{}{}{}{}", " ".repeat(n), "*", "/".repeat(27-n), "\\".repeat(27-n), "*");
        //Add the spaces needed before the n
    }
}