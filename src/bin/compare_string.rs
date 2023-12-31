///compare strings in directionary order
fn compare_string(x:&str,y:&str)->bool{
    let charsx:Vec<char> = x.chars().collect(); //convert to vec for the sake of convenience
    let charsy:Vec<char> = y.chars().collect();
    let mut ptr = 0; //the index
    while ptr<charsx.len() && ptr<charsy.len(){
        if charsx[ptr]>charsy[ptr]{
            return true; //return directly
        }else if charsx[ptr]<charsy[ptr]{
            return false;
        }
        ptr += 1;
    }
    if ptr==charsx.len(){
        return false;
    }else if ptr==charsy.len(){
        return true;
    }else{
        return false; //when equal return false
    }
}

fn main(){
    let string1 = "abc";  //for test
    let string2 = "abcd";
    let result1 = compare_string(string1, string2);
    assert_eq!(result1,false);
    println!("The result of comparing {} and {} is: {}",string1,string2,result1);
    
    let string3 = "mnt";
    let result2 = compare_string(string3, string1);
    assert_eq!(result2,true);
    println!("The result of comparing {} and {} is: {}",string3,string1,result2);
}