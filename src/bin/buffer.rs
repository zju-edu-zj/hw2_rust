
struct Buffer<T>{
    member:Vec<T>
}
impl<T: std::iter::Sum> Buffer<T> {
    
    fn sum(&self) -> T
    where
        T: std::ops::Add<Output = T>  + Clone, //here we should bound T
    {
        self.member.iter().cloned().sum()  //we just call the sum method directly
    }
}


fn main(){
    //integer
    let vec1 = vec![1,3,5]; //the sum is 9
    let buffer1 = Buffer{member:vec1};
    println!("The sum of vec1 is {}",buffer1.sum());
    
    //float
    let vec2 = vec![1.2,3.3,5.5,6.6]; //the sum is 16.6
    let buffer2 = Buffer{member:vec2};
    println!("The sum of vec2 is {}",buffer2.sum());
}