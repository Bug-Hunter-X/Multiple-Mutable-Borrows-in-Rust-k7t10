fn main() {
    let mut x = 5;
    { //This block ensures that only one mutable reference exists at any given time
        let y = &mut x;
        *y += 1;
    }
    { //This block ensures that only one mutable reference exists at any given time
        let z = &mut x;
        *z += 1;
    }
    println!("x = {}", x);
}