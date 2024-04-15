fn is_positive_integer_present(arr: &Vec<i32>) -> Option<i32> {
    let n = arr.len();
    let mut min_positive = std::i32::MAX;
    
    for i in 0..n {
        if arr[i] > 0 && arr[i] < min_positive {
            min_positive = arr[i];
        }
    }
    
    if min_positive != std::i32::MAX {
        return Some(min_positive);
    }
    else 
    {
        return None;
    }
}





fn main() {
    let arr = vec![-1, -2, -3, 0, -5, -6];
  
    match is_positive_integer_present(&arr) {
        Some(val) => println!("Smallest positive integer is: {}", val),
        None => println!("No number is bigger than 0"),
    }
}
