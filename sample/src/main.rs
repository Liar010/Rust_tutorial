fn main() {

    //what is Vector? -> resizable array

    let v0:Vec<u8> = Vec::new();
    let mut v1 = vec![0, 1, 2];
    let v2 = vec![1; 3];

    println!("length v0:{}, v1:{}, v2:{}", v0.len(), v1.len(), v2.len());

    v1.push(3);//Append value to end of vector

    for i in 0..v1.len(){
        println!("v1[{}] = {}", i, v1[i]);
    }
    v1.pop();//pop values from end of vector

    for (i, el) in v1.iter().enumerate(){
        println!("v1[{}] = {}", i, el);
    }

    /*other usage
        for el in v1 {
            println!("v1={}", el);
        }
        for el in v1.iter(){
            println!("v1={}", el)
        }
    */
}