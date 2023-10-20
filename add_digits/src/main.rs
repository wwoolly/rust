fn main() {
    println!("Hello, world! {}", add_digits(123));
}

fn add_digits(num: i32) -> i32 {
    let mut x = num;
    let mut acc = 0;
    
    return loop { 
        let mut counter = 0;
        loop {
            counter += 1;
            acc += x % 10;
            x = x / 10;
            
            if x == 0 {
                break;
            }
        }

        if counter <= 1 {
            break acc;
        }

        x = acc;
        acc = 0;
    }        
}

#[test]
fn test_sum_of_2() {
    assert_eq!(add_digits(2), 2);
}

#[test]
fn test_sum_of_0() {
    assert_eq!(add_digits(0), 0);
}

#[test]
fn test_sum_of_32() {
    assert_eq!(add_digits(32), 5);
}    

#[test]
fn test_sum_of_1000() {
    assert_eq!(add_digits(1000), 1);
}

#[test]
fn test_sum_of_789() {
    assert_eq!(add_digits(789), 6);
}    

#[test]
fn test_sum_of_487() {
    assert_eq!(add_digits(487), 1);
}    