use std::vec;

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



    //test merge sort
    let mut arr_to_merge = vec![5,4,3,2,1]; 
    let sorted_by_merge = sorting::merge_sort(&mut arr_to_merge); 
    println!("merge sort results {:?}", sorted_by_merge);

    //test quick sort
    let mut arr_quick = vec![5,3,4,2,1];
    let n = arr_quick.len();
    sorting::quick_sort(&mut arr_quick, 0, n - 1);
    println!("quick sort results {:?}", arr_quick);

}
