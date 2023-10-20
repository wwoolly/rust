fn main() {
    println!("Да уж! {}", is_power_of_two(2));
}

fn is_power_of_two(n: i32) -> bool {
    if n == 0 { return false; }

    let mut digit = n;
    let mut module = 0;
    loop {
        if digit == 1 || digit == 0 {
            break;
        }
        
        module = digit % 2;
        digit = digit / 2;
    }

    return module == 0;
}
    
#[test]
fn test_is_power_of_two_0() {
    assert!(!is_power_of_two(0));
}

#[test]
fn test_is_power_of_two_1() {
    assert!(is_power_of_two(1));
}

#[test]
fn test_is_power_of_two_24() {
    assert!(!is_power_of_two(24));
}

#[test]
fn test_is_power_of_two_1003() {
    assert!(!is_power_of_two(1003));
}

#[test]
fn test_is_power_of_two_2() {
    assert!(is_power_of_two(2));
}

#[test]
fn test_is_power_of_two_1024() {
    assert!(is_power_of_two(1024));
}
