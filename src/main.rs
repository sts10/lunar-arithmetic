fn main() {
    let a = 17;
    let b = 26;
    println!("On the moon, 17 + 26 is {}", lunar_add(a, b));
    println!("On the moon, 58 + 19 is {}", lunar_add(58, 19));
    println!("On the moon, 15866 + 147 is {}", lunar_add(15866, 147));
}

fn lunar_add(a: usize, b: usize) -> usize {
    let (a, b) = vectorize_and_pad(a, b);

    let mut sum_digits: Vec<u32> = [].to_vec();

    // doing the lunar addition
    for (i, _digit) in a.iter().enumerate() {
        if a[i] > b[i] {
            sum_digits.push(a[i]);
        } else {
            sum_digits.push(b[i]);
        }
    }

    vector_of_digits_to_int(sum_digits)
}

fn lunar_multiply(a: usize, b: usize) -> usize {
    let (mut a, mut b) = vectorize_and_pad(a, b);
    let mut numbers_to_add: Vec<usize> = [].to_vec();

    a.reverse();
    b.reverse();

    let mut i = 0;
    for b_dig in b {
        let mut product_digits: Vec<u32> = [].to_vec();

        for a_dig in &a {
            if b_dig < *a_dig {
                product_digits.insert(0, b_dig);
            } else {
                product_digits.insert(0, *a_dig);
            }
        }
        let mut j = 0;
        while j < i {
            product_digits.push(0);
            j += 1;
        }
        numbers_to_add.push(vector_of_digits_to_int(product_digits));
        i += 1;
    }
    // println!("numbers_to_add is current {:?}", numbers_to_add);

    let mut running_sum: usize = 0;

    for num in numbers_to_add {
        running_sum = lunar_add(running_sum, num);
    }
    running_sum
}

fn vectorize_and_pad(a: usize, b: usize) -> (Vec<u32>, Vec<u32>) {
    let mut a: Vec<_> = a
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();
    let mut b: Vec<_> = b
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();

    // pad the shorter vector
    while a.len() < b.len() {
        a.insert(0, 0);
    }
    while b.len() < a.len() {
        b.insert(0, 0);
    }
    (a, b)
}

fn vector_of_digits_to_int(digits_vec: Vec<u32>) -> usize {
    let mut vector_of_string: Vec<String> = [].to_vec();
    for dig in digits_vec {
        vector_of_string.push(dig.to_string());
    }

    // now make sum_digits back into a usize
    let as_string: String = vector_of_string.into_iter().collect();
    let num: usize = as_string.parse().unwrap();
    num
}
#[test]
fn can_do_lunar_addition() {
    assert_eq!(lunar_add(7, 8), 8);
    assert_eq!(lunar_add(17, 26), 27);
    assert_eq!(lunar_add(58, 19), 59);
    assert_eq!(lunar_add(19, 58), 59);
    assert_eq!(lunar_add(169, 248), 269);
}
#[test]
fn can_lunar_add_numbers_of_different_number_of_digits() {
    assert_eq!(lunar_add(123, 45), 145);
}

#[test]
fn can_do_simple_lunar_multiplication() {
    assert_eq!(lunar_multiply(7, 8), 7);
}
#[test]
fn can_do_double_and_single_digit_lunar_multiplication() {
    assert_eq!(lunar_multiply(26, 3), 23);
}
#[test]
fn can_do_lunar_multiplication() {
    assert_eq!(lunar_multiply(17, 24), 124);
    assert_eq!(lunar_multiply(169, 248), 12468);
}
#[test]
fn can_lunar_multiply_numbers_of_different_number_of_digits() {
    assert_eq!(lunar_multiply(15866, 147), 1145766);
}
