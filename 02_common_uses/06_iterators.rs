
fn duplicate_vector(v: &mut Vec<u32>){
    for n in v{
        v.push(*n);
    }

}


fn main(){
    let mut v = vec![1,2,3,4,5,6,7,8,9,10,11];
    duplicate_vector(&mut v);
    println!("vec: {:?}", v);

}

