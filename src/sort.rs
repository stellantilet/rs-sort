pub fn quick_sort(arr: &mut [i32]) {
    if !arr.is_empty() {
        let mut sep = 0;
        for i in 1..arr.len() {
            if arr[i] < arr[0] {
                sep += 1;
                arr.swap(sep, i);
            }
        }

        arr.swap(0, sep);
        quick_sort(&mut arr[..sep]);
        quick_sort(&mut arr[(sep + 1)..]);
    }
}

pub fn bubble_sort(arr: &mut [i32]) {
    let mut new_len: usize;
    let mut len = arr.len();
    loop {
        new_len = 0;
        for i in 1..len {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                new_len = i;
            }
        }
        if new_len == 0 {
            break;
        }
        len = new_len;
    }
}

pub fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

pub fn heap_sort(arr: &mut [i32]) {
    // -- Heapify part --
    // This procedure would build a valid max-heap.
    // (or min-heap for sorting descendantly)
    let end = arr.len();
    for start in (0..end / 2).rev() {
        // Skip leaf nodes (end / 2).
        sift_down(arr, start, end - 1);
    }

    // -- Sorting part --
    // Iteratively sift down unsorted part (the heap).
    for end in (1..arr.len()).rev() {
        arr.swap(end, 0);
        sift_down(arr, 0, end - 1);
    }
}

fn sift_down(arr: &mut [i32], start: usize, end: usize) {
    let mut root = start;
    loop {
        let mut child = root * 2 + 1; // Get the left child
        if child > end {
            break;
        }
        if child < end && arr[child] < arr[child + 1] {
            // Right child exists and is greater.
            child += 1;
        }

        if arr[root] < arr[child] {
            // If child is greater than root, swap'em!
            arr.swap(root, child);
            root = child;
        } else {
            break;
        }
    }
}

pub fn selection_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        let mut temp = i;
        for j in (i + 1)..len {
            if arr[temp] > arr[j] {
                temp = j;
            }
        }
        arr.swap(i, temp);
    }
}
