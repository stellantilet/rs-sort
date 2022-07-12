use std::io;
use std::io::BufRead;
pub mod sort;

fn main() {
    let stdin = io::stdin();

    let mut arr: [i32; 8] = [342, 3, 23, 34, 129, 1024, 3074, 523];
    sort::quick_sort(&mut arr);

    let arr_str: Vec<String> = arr.iter().map(|x| x.to_string()).collect();
    println!("Quick Sort Result: {}", arr_str.join(" "));

    // QUICK SORT
    let mut arr: Vec<i32> = match stdin.lock().lines().nth(0) {
        Some(Ok(s)) => s.split(' ').flat_map(|s| s.parse()).collect(),
        _ => unreachable!(),
    };
    sort::quick_sort(&mut arr);

    let arr_str: Vec<String> = arr.iter().map(|x| x.to_string()).collect();
    println!("Quick Sort Result: {}", arr_str.join(" "));

    // BUBBLE SORT
    let mut arr: Vec<i32> = match stdin.lock().lines().nth(0) {
        Some(Ok(s)) => s.split(' ').flat_map(|s| s.parse()).collect(),
        _ => unreachable!(),
    };
    sort::bubble_sort(&mut arr);

    let arr_str: Vec<String> = arr.iter().map(|x| x.to_string()).collect();
    println!("Bubble Sort Result: {}", arr_str.join(" "));

    // INSERTION SORT
    let mut arr: Vec<i32> = match stdin.lock().lines().nth(0) {
        Some(Ok(s)) => s.split(' ').flat_map(|s| s.parse()).collect(),
        _ => unreachable!(),
    };
    sort::insertion_sort(&mut arr);

    let arr_str: Vec<String> = arr.iter().map(|x| x.to_string()).collect();
    println!("Insertion Sort Result: {}", arr_str.join(" "));

    // HEAP SORT
    let mut arr: Vec<i32> = match stdin.lock().lines().nth(0) {
        Some(Ok(s)) => s.split(' ').flat_map(|s| s.parse()).collect(),
        _ => unreachable!(),
    };
    sort::heap_sort(&mut arr);

    let arr_str: Vec<String> = arr.iter().map(|x| x.to_string()).collect();
    println!("Heap Sort Result: {}", arr_str.join(" "));

    // SELECTION SORT
    let mut arr: Vec<i32> = match stdin.lock().lines().nth(0) {
        Some(Ok(s)) => s.split(' ').flat_map(|s| s.parse()).collect(),
        _ => unreachable!(),
    };
    sort::selection_sort(&mut arr);

    let arr_str: Vec<String> = arr.iter().map(|x| x.to_string()).collect();
    println!("Selection Sort Result: {}", arr_str.join(" "));
}
