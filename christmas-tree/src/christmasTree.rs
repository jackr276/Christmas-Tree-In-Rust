/**
 * @author Jack Robbins
 * 
 * In the festive spirit, this project simply prints an ASCII christmas tree to 
 * the terminal 
*/


fn main() {
    println!("                           *                          ");

    christmas_tree()
}




fn christmas_tree(){
    for n in 1..20{
        println!("{}",n)
    }
}