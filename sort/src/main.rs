fn main() {
    println!("Hello, world!");
    
    sort(&mut [1, 3, 2], |a, b| a < b);
}

fn sort(arr : &mut [i32], cmp: fn(&i32, &i32) -> bool) {
    let len = arr.len();
    let mut swap_flag;
    
    for i in 0..len - 1 {
        swap_flag = false;

        for j in 0..len - 1 - i {
            if cmp(&arr[j], &arr[j + 1]) {
                arr.swap(j, j + 1);
                swap_flag = true;
            }
        }
        if !swap_flag {
            break;
        }
    }
}

#[test]
fn sort_test() {
    let mut arr1 = [2, 5, 3, 4, 1];

    sort(&mut arr1, |a, b| a > b);
    assert_eq!([1, 2, 3, 4, 5], arr1);

    sort(&mut arr1, |a, b| a < b);
    assert_eq!([5, 4, 3, 2, 1], arr1);


    let mut arr2 = [1, 2, 3, 3, 5, 6];
    sort(&mut arr2, |a, b| a > b);
    assert_eq!([1, 2, 3, 3, 5, 6], arr2);

    sort(&mut arr2, |a, b| a < b);
    assert_eq!([6, 5, 3, 3, 2, 1], arr2);

    sort(&mut arr2, |a, b| a > b);
    assert_ne!([6, 5, 3, 3, 2, 1], arr2);
}
