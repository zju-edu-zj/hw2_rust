fn main(){
    let vector = vec!['a','b','c','d','e'];
    let new_vector:Vec<char> = vector.iter().map(|&x| (x as u8 +1) as char).collect();
    println!("{:?}",new_vector); //? to express vector
}