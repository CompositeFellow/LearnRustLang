// First challenge from book Algorithms Illuminated.  
//Implimenting my own mergeSort algorithm.


fn merge_sort(unsorted_vec: &Vec<i32>) -> Vec<i32> {
    if unsorted_vec.len() < 2 { return unsorted_vec.to_vec(); }
    
    let split_point = unsorted_vec.len() / 2; 
    let first_half: Vec<i32> = merge_sort(&unsorted_vec[..split_point].to_vec());
    let second_half: Vec<i32> = merge_sort(&unsorted_vec[split_point..].to_vec());
    let merged = merge(&first_half, &second_half);

    return merged;
}


fn merge (left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32>{
    let mut merged: Vec<i32> = Vec::new();
    let mut left_i = 0;
    let mut right_i = 0;

    while left_i < left.len() && right_i < right.len(){
        if left[left_i] <= right[right_i]{
            merged.push(left[left_i]);
            left_i += 1;
        } else {
            merged.push(right[right_i]);
            right_i += 1;
        }
    }

    //if left is larger than right catch
    while left_i < left.len() {
        merged.push(left[left_i]);
        left_i += 1;
    }

    while right_i < right.len() {
        merged.push(right[right_i]);
        right_i += 1;
    }

    merged
}

pub fn run_merge_sort(){
    let  test_1: Vec<i32> = vec![5,3,9,0,2,1,8,7];
    let  test_2: Vec<i32> = vec![5,3,1,8,7];
    let  test_3: Vec<i32> = vec![0,2,1,8,7,3,7,5,3,9,4,6];
    let one: Vec<i32> = merge_sort(&test_1);
    let two: Vec<i32> = merge_sort(&test_2);
    let three: Vec<i32> = merge_sort(&test_3);

    println!("Before: {:?} , After: {:?}",&test_1, &one);
    println!("Before: {:?} , After: {:?}",&test_2, &two);
    println!("Before: {:?} , After: {:?}",&test_3, &three);
}