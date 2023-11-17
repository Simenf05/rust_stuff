


fn check_num(num: u64, primes: Vec<u64>) -> bool {
    for prime in primes {
        if num % prime == 0 {
            return false;
        }

    }

    true
}


pub fn run(max_num: u64) {

    
    let mut prime_vec: Vec<u64> = Vec::new();

    for i in 2..max_num {

        if check_num(i, prime_vec.clone()) {
            prime_vec.push(i);
        }

    }

    println!("{:?}", prime_vec);

}


