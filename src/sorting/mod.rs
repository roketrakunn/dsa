pub fn bubble_sort(arr:  &mut Vec<i32>) {
    
    for _ in 0..arr.len() { 

        let mut swapped = false;

        for i in 0..arr.len()-1 { 
            if arr[i]  > arr[i+1] { 
                arr.swap(i, i+1);
                swapped = true
            }
        }

        if swapped == false { 
            break;
        }
    }
}
// merge sort
// -divide the array in half , sor each half then marge  them back togther in order
// recursive algorithm btw(yep yo ucan suck it). 

pub fn merge_sort(arr : &[i32]) -> Vec<i32> {
    let n = arr.len();

    if n <= 1 { 
        return arr.to_vec();
    }
    
    let left : Vec<i32> = arr[0..n/2].to_vec(); 
    let right : Vec<i32> = arr[n/2..].to_vec(); 

    merge(&merge_sort(&left), &merge_sort(&right))
}

//helper to merge two arrays
pub fn merge(left: &[i32], right : &[i32]) -> Vec<i32> {
    let mut  res : Vec<i32> = Vec::new();
    let mut i = 0; 
    let mut j = 0; 

    while i < left.len() && j <  right.len() {

        if left[i] < right[j] { 
            res.push(left[i]);
            i += 1;
        } else { 
            res.push(right[j]);
            j += 1;
        }
    }
    res.extend_from_slice(&left[i..]);
    res.extend_from_slice(&right[j..]);

    res
}

pub fn quick_sort(arr: &mut [i32], low: usize , high: usize) {

    if low >= high  { 
        return;
    }

    let pivot = partition(arr, low, high);

    if pivot > 0 { 
        quick_sort(arr, low, pivot -1);
    }
    quick_sort(arr, pivot+1, high);

}

fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut boundary = low;

    for scanner in low..high {
        if arr[scanner] < pivot {
            arr.swap(boundary, scanner);
            boundary += 1;
        }
    }
    arr.swap(boundary, high);
    boundary
}


