/**
 * @author Jack Robbins
 * 
 * In the festive spirit, this project simply prints an ASCII christmas tree to 
 * the terminal 
*/

//External Crate for text formatting
use colored::*;

fn main() {
    println!("{}{}{}{}", "\n", " ".repeat(20), "Merry Christmas!".truecolor(214,0,28).bold(), "\n");
    christmas_tree();
    
    //print out the trunk
    println!("{}{}", " ".repeat(27), "||".truecolor(101,67,33).bold());
    println!("{}{}{}", " ".repeat(27), "||".truecolor(101,67,33).bold(), "\n");
}

//Uses a reverse for loop to appropriately print out the "tree", asterisk ornaments and needed spaces
fn christmas_tree(){
    for n in (0..27).rev(){
        println!("{}{}{}{}{}", " ".repeat(n), "*".yellow().bold(), "/".repeat(27-n).truecolor(0,110,51).bold(), "\\".repeat(27-n).truecolor(0,110,51).bold(), "*".yellow().bold());
    }
}