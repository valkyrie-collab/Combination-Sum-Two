use std::{
    io::{
        self,
        Write,
    },
};

fn find_combo(arr: &[i32], target: i32, idx: usize, total: i32, combo: &mut Vec<Vec<i32>>, cur_combo: &mut Vec<i32>) {
    if total == target {
        combo.push(cur_combo.clone());
        return;
    }

    if idx >= arr.len() || total > target {
        return;
    }

    cur_combo.push(arr[idx]);
    find_combo(arr, target, idx + 1, total + arr[idx], combo, cur_combo);
    cur_combo.pop();

    let mut temp: usize = idx;

    while arr[idx] == arr[temp] {
        temp += 1;

        if temp >= arr.len() {
            break;
        }
    }

    find_combo(arr, target, temp, total, combo, cur_combo);
}

fn merge(arr: &mut [i32], left: i32, right: i32, mid: i32) {
    let len_one: usize = (mid - left + 1) as usize;
    let len_two: usize = (right - mid) as usize;

    let mut arr_one: Vec<i32> = vec![-1; len_one];
    let mut arr_two: Vec<i32> = vec![-1; len_two];

    for x in 0..len_one {
        arr_one[x] = arr[left as usize + x]
    }

    for x in 0..len_two {
        arr_two[x] = arr[(mid + 1) as usize  + x]
    }

    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut z: usize = left as usize;

    while x < len_one && y < len_two {

        if arr_one[x] <= arr_two[y] {
            arr[z] = arr_one[x];
            x += 1;
        } else {
            arr[z] = arr_two[y];
            y += 1;
        }

        z += 1;
    }

    while x < len_one {
        arr[z] = arr_one[x];
        z += 1;
        x += 1;
    }

    while y < len_two {
        arr[z] = arr_two[y];
        z += 1;
        y += 1;
    }
}

fn sort(arr: &mut [i32], left: i32, right: i32) {
    if left >= right {
        return;
    }

    let mid: i32 = left + (right - left) / 2;

    sort(arr, left, mid);
    sort(arr, mid + 1, right);
    merge(arr, left, right, mid);
}

fn main() {
    let mut buf: String = String::new();

    print!("Enter the size of array: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buf).unwrap();

    let size: usize = if let Ok(x) = buf.trim().parse() {x} else {0};
    let mut arr: Vec<i32> = vec![-1; size];

    println!("Enter the elements of array below: ");
    for x in 0..size {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        arr[x] = if let Ok(xi) = buf.trim().parse() {xi} else {continue};
    }

    buf.clear();
    print!("Enter the target: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buf).unwrap();

    let target: i32 = if let Ok(x) = buf.trim().parse() {x} else {-1};

    if target == -1 {
        println!("provide a valid input ");
        return;
    }

    // let mut arr: Vec<i32> = vec![10,1,2,7,6,1,5];
    // let target: i32 = 8;
    // let size: i32 = arr.len() as i32;
    sort(&mut arr, 0, (size - 1) as i32);
    println!("Sorted array: {:?}", &arr);

    let mut combo: Vec<Vec<i32>> = Vec::new();
    let mut cur_combo: Vec<i32> = Vec::new();

    find_combo(&arr, target, 0, 0, &mut combo, &mut cur_combo);
    println!("{:?}", combo);
}
