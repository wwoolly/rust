fn main() {
    println!("Hello, world! {}", is_palindrome(121));
}

fn is_palindrome(num: i32) -> bool {
    //пограничные условия для упрощения основного аглоритма
    if num < 0 {
        return false;
    } 
    else if num < 10 {
        return true;
    }

    let x : u64 = num as u64;

    //the variable multiple of 10 that can divide the 'x' without reminder  
    let mut divider : u64 = 1;
    //число разрядов
    let mut digit_counter = 0;
    loop {
        if x / divider == 0 {
            break;
        }

        divider *= 10u64;
        digit_counter += 1;
    }

    //далее выступает как делитель для нахождения старших разрядов
    divider /= 10;

    //делитель младших разрядов 
    let mut tail_divider = 10;
    //храним крайний левый и крайний правый разряды числа, сравнивая их сдвигаемся к середине 
    let mut border_values : (u64, u64) = (0, 0);
    //счётчик сдвига к середине
    let mut position = 0;
    return loop {
        position += 1;
        border_values.0 = (x / divider) % 10u64 as u64;
        border_values.1 = (x % tail_divider) / (tail_divider / 10);

        if border_values.0 != border_values.1
        {
            break false;
        }
        else if digit_counter / 2 <= position {
            break true;
        }

        divider /= 10;
        tail_divider *= 10; 
    }
}

#[test]
fn test_sym_121() {
    assert!(is_palindrome(121));
}

#[test]
fn test_sym_123() {
    assert!(!is_palindrome(123));
}

#[test]
fn test_sym_0() {
    assert!(is_palindrome(0));
}

#[test]
fn test_sym_minus_121() {
    assert!(!is_palindrome(-121));
}

#[test]
fn test_sym_1() {
    assert!(is_palindrome(1));
}

#[test]
fn test_sym_100001() {
    assert!(is_palindrome(100001));
}

#[test]
fn test_sym_987789() {
    assert!(is_palindrome(987789));
}

#[test]
fn test_sym_987189() {
    assert!(!is_palindrome(987189));
}

#[test]
fn test_sym_1234554321() {
    assert!(is_palindrome(1234554321));
}