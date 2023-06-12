use memoize::memoize;

const NUM_OF_EXPS: usize = 129;

#[memoize]
fn f_real_count(n: u32) -> u128 {
    if n == 0 { 
        0
    } else {
        (0..n).map(|i| f_real_count(i) + 1).sum::<u128>()
    }
}

fn f_theoretical_count(n: u32) -> u128 {
    2u128.pow(n) - 1
}

pub fn ex15() {
    println!("ex15");
    for n in 0..NUM_OF_EXPS {
        let real_count = f_real_count(n as u32);
        let theoretical_count = f_theoretical_count(n as u32);
        println!("n = {}, real_count = {}, theoretical_count = {}", n, real_count, theoretical_count);
        if real_count != theoretical_count {
            println!("Error: real_count != theoretical_count");
            break;
        }
    }
}

#[memoize]
fn l_real_count(n: u32) -> u128 {
    match n {
        0 | 1 => 1,
        n => {
            let prev_sum = (1..n).map(|i| l_real_count(i)).sum::<u128>();
            prev_sum + 2
        }
    }
}

fn l_rand_count(n: u32) -> u128 {
    match n {
        0 | 1 => 1,
        n => {
            let mut count = 1;
            for i in 1..=n {
                if fastrand::bool() {
                    count += l_rand_count(i);
                }
            }
            count
        }
    }
}

fn l_rand_count_avg(n: u32, num_of_tests: usize) -> u128 {
    let mut sum = 0;
    for _ in 0..num_of_tests {
        sum += l_rand_count(n);
    }
    sum / num_of_tests as u128
}


fn l_theoretical_count(n: u32) -> u128 {
    match n {
        0 | 1 => 1,
        n => {
            3 * 2u128.pow(n - 2)
        }
    }
}

pub fn ex16() {
    println!("ex16");
    for n in 0..NUM_OF_EXPS {
        let real_count = l_real_count(n as u32);
        let theoretical_count = l_theoretical_count(n as u32);
        println!("n = {}, real_count = {}, theoretical_count = {}", n, real_count, theoretical_count);
        if real_count != theoretical_count {
            println!("Error: real_count != theoretical_count");
            break;
        }
    }
}

pub fn ex16rand() {
    println!("ex16 - randomized");
    let rand_count = l_rand_count_avg(10, 1_000_000);
    println!("rand_count for n = 10: {}", rand_count);
}