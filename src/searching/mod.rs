pub fn binary_search(arr: &[i32] , target: i32) -> Option<usize> {

    let mut left : usize = 0 ; 
    let mut right = arr.len() -1;

    let mut mid = arr.len()/2;

    while left < right  {

        if target == arr[mid] { 
            return Some(mid);
        }

        if target < arr[mid] { 
            right = mid -1;
        }
        
        if target > arr[mid] { 
            left  = mid + 1;
        }

        mid = (left + right) / 2;
    }

    None
}

