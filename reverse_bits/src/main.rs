fn main() {
    println!("Reversed 123 is {}", reverse_bits(123u32));
}

fn reverse_bits(x: u32) -> u32 { 
    let mut num = 0;

    for i in 0..32 {
        num |= ((x << i) >> 31) << i;
    }

    return num;
}

#[test]
fn test_reverse_43261596() {
    assert_eq!(reverse_bits(43261596u32), 964176192u32);
}

#[test]
fn test_reverse_1() {
    assert_eq!(reverse_bits(0b0000_0000_0000_0000_0000_0000_0000_0001u32), 0b1000_0000_0000_0000_0000_0000_0000_0000u32);
}

#[test]
fn test_reverse_2() {
    assert_eq!(reverse_bits(0b0010_0100_0000_0000_0000_0000_0000_0001u32), 0b1000_0000_0000_0000_0000_0000_0010_0100u32);
}

#[test]
fn test_reverse_3() {
    assert_eq!(reverse_bits(0b1111_1111_1111_1111_0000_0000_0000_0000u32), 0b0000_0000_0000_0000_1111_1111_1111_1111u32);
}

#[test]
fn test_reverse_4() {
    assert_eq!(reverse_bits(0b0000_0000_0000_0000_1111_1111_1111_1111u32), 0b1111_1111_1111_1111_0000_0000_0000_0000u32);
}

#[test]
fn test_reverse_5() {
    assert_eq!(reverse_bits(0b1010_1010_1010_1010_1010_1010_1010_1010u32), 0b0101_0101_0101_0101_0101_0101_0101_0101u32);
}

#[test]
fn test_reverse_6() {
    assert_eq!(reverse_bits(0b0101_0101_0101_0101_0101_0101_0101_0101u32), 0b1010_1010_1010_1010_1010_1010_1010_1010u32);
}