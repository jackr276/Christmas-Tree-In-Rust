/**
 * @author Jack Robbins
 * 
 * In the festive spirit, this project simply prints an ASCII christmas tree to 
 * the terminal 
*/

use colored::*;

fn main() {
    println!("\n                    Merry Christmas!!\n");
    christmas_tree();
    println!();
}

fn christmas_tree(){
    for n in (0..27).rev(){
        println!("{}{}{}{}{}", " ".repeat(n), "*".yellow(), "/".repeat(27-n).green(), "\\".repeat(27-n).green(), "*".yellow());
        //Add the spaces needed before the n
    }
}