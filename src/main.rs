mod sorting;
mod searching;
fn main() {

    let mut arr = vec![5,3,8,1]; 
    
    sorting::bubble_sort(&mut arr);
    println!("{:?}", arr);

    let mut arr_sorted = vec![1,2,3,4,5]; 
    sorting::bubble_sort(&mut arr_sorted);
    println!("{:?}", arr_sorted);
    // binary searching and all.

    let search_arr = vec![1,2,3,4,5,5,6,6];
    let found_at = searching::binary_search(&search_arr,5);

    match found_at {
        Some(index) => println!("found at:  {}",index),
        None => println!("not found"),
    }
}
