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


