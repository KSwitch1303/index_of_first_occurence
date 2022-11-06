fn main() {
    println!("{:?}",codde(String::from("leetcode"), String::from("leeto")));
}
pub fn codde(hay: String,pin: String) -> i32{
    let mut index: i32= -1;
    for i in 0..hay.len(){
        if hay.chars().nth(i) == pin.chars().nth(0){
            let temp_hold: String =hay.chars().skip(0+i).take(pin.len()).collect();
            println!("{}",temp_hold);
            if temp_hold == pin{
                println!("success");
                index = i as i32;
                break;
            }
        }
    }
    return index;
}
