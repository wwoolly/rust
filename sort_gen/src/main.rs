use rand::Rng;
use std::fmt;

fn main() {
    println!("Hello, world!");
}

fn sort<T>(arr : &mut [T], cmp: fn(&T, &T) -> bool) {
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

// #[derive(Clone, Copy)]e
struct Element<T, U> {
    re : T,
    im : U
}

impl <T : PartialEq, U : PartialEq> PartialEq for Element<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.re == other.re && self.im == other.im
    }
}

impl <T : PartialEq, U : PartialEq> Eq for Element<T, U> {}

impl <T : fmt::Display, U : fmt::Display> fmt::Display for Element<T, U> {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result { 
        write!(f, "re = {}; im = {};", self.re, self.im)
    }
}

impl <T, U> Element<T, U> {
    fn new(re: T, im: U) -> Self {
        Self { re, im }
    }
}

impl Element<i32, i32> {
    fn default() -> Self {
        let re = rand::thread_rng().gen_range(0..100);
        let im = rand::thread_rng().gen_range(0..100);

        Self { re, im }
    }
}

#[test]
fn test_sorting() {
    let mut arr1: [Element<i32, f32>; 3] = test_array_1_1();
    
    sort(&mut arr1, |a : &Element<i32, f32>, b : &Element<i32, f32>| a.im > b.im);
    assert!(compare_arrays(&sorted_array_1(), &arr1));

    arr1 = test_array_1_2();
    sort(&mut arr1, |a : &Element<i32, f32>, b : &Element<i32, f32>| a.im > b.im);
    assert!(compare_arrays(&sorted_array_1(), &arr1));
}

// это не идеальная версия 
fn compare_arrays<T : PartialEq, U : PartialEq>(arr1 : &[Element<T, U>], arr2 : &[Element<T, U>]) -> bool
    where T: PartialEq, 
    T : fmt::Display,
    U : PartialEq,
    U : fmt::Display,
    {
    if arr1.len() != arr2.len() {
        println!("Arrays length is not equal ({} != {})", arr1.len(), arr2.len());
        return false;
    }

    for i in 0..arr1.len() {
        if arr1[i] != arr2[i] {
            println!("Elements with index '{}' is not equal! -- {} != {}", i, arr1[i], arr2[i]);
            return false;
        }
    }
    return true;
}

fn test_array_1_1() -> [Element<i32, f32>; 3] {
    return [
        Element { re: (3i32), im: (4f32) },
        Element { re: (1i32), im: (2f32) },
        Element { re: (2i32), im: (3f32) }
    ];
}

fn test_array_1_2() -> [Element<i32, f32>; 3] {
    return [
        Element { re: (2i32), im: (3f32) },
        Element { re: (3i32), im: (4f32) },
        Element { re: (1i32), im: (2f32) }
    ];
}

fn sorted_array_1() -> [Element<i32, f32>; 3] {
    return [
        Element { re: (1i32), im: (2f32) },
        Element { re: (2i32), im: (3f32) },
        Element { re: (3i32), im: (4f32) }
    ];
}