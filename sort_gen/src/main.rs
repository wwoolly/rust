use rand::Rng;

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

impl <T, U> Element<T, U> {
    fn new(re: T, im: U) -> Self {
        Self { re, im }
    }
}


#[derive(Copy)]
impl Element<i32, i32> {
    fn default() -> Self {
        let re = rand::thread_rng().gen_range(0..100);
        let im = rand::thread_rng().gen_range(0..100);

        Self { re, im }
    }
}

#[test]
fn test_sorting() {
    let mut arr1 : [Element<i32, f32>; 3] = [
        Element { re: (1i32), im: (2f32) },
        Element { re: (2i32), im: (3f32) },
        Element { re: (3i32), im: (4f32) }
    ];

    sort(&mut arr1, |a : &Element<i32, f32>, b : &Element<i32, f32>| a.im > b.im);
    assert_eq!([
        Element { re: (1i32), im: (2f32) },
        Element { re: (2i32), im: (3f32) },
        Element { re: (3i32), im: (4f32) }
    ], arr1);

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

// это не идеальная версия 
fn compare_arrays<T, U>(arr1 : &mut [Element<T, U>], arr2 : &mut [Element<T, U>]) -> bool {
    assert_eq!(arr1.len(), arr2.len());

    for i in 0..arr1.len() {
        if arr1[i] != arr2[i] {
            panic!("Elements with index '{}' is not equal!", i);
        }
    }
    return true;
}

fn test_array_1() -> [Element<i32, f32>; 3] {
    return [
        Element { re: (3i32), im: (4f32) },
        Element { re: (1i32), im: (2f32) },
        Element { re: (2i32), im: (3f32) }
    ];
}

fn sorted_array_1() -> [Element<i32, f32>; 3] {
    return [
        Element { re: (1i32), im: (2f32) },
        Element { re: (2i32), im: (3f32) },
        Element { re: (3i32), im: (4f32) }
    ];
}