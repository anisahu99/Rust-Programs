fn main() {
    let is_male: bool   = true;
    let is_above_18: bool = true;

    if is_male {
        println!("You are a male");
    }else{
        println!("You are not a male");
    }

    if is_male && is_above_18 {
        println!("You are adult male");
    }
}
