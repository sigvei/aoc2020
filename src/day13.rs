use modinverse::egcd;

fn waitingtime(now: u64, routes: &str) -> (u64, u64) {
    let mut minwait = now;
    let mut minwaitroute = 0;

    for route in routes.split(',') {
        if route != "x" {
            let num = u64::from_str_radix(route, 10).unwrap();
            let wait = (num - (now % num)) % num;
            // modulo here is the least find the first multiple of num > now
            if minwait > wait {
                minwait = wait;
                minwaitroute = num;
            }
        }
    }
    (minwait, minwaitroute)
}

fn departtime(routes: &str) -> i128 {
    let mut routes: Vec<(i128, i128)> = routes
        .split(',')
        .into_iter()
        .enumerate()
        .filter(|(_, v)| v != &"x")
        .map(|(i, v)| (i as i128, i128::from_str_radix(v, 10).unwrap()))
        .map(|(i, v)| ((v - i) % v, v))
        .collect();

    println!("{:?}", routes);

    // Implement the chinese remainder theorem, where a_i are the indices
    // (waiting times) and n_i are the moduli (route numbers)
    let (mut a1, mut n1) = routes.pop().unwrap();
    let mut solution: i128 = 0;
    // iteratively determine the solution for the two equations
    // x = a0 mod n0
    // x = a1 mod n1
    // by using the chinese remainder theorem, substituting the solution
    // a_{0,1} mod n0n1 for a0 and n0 as we walk through the constraints
    while !routes.is_empty() {
        let (a2, n2) = routes.pop().unwrap();
        let (_, m1, m2) = egcd(n1, n2);
        solution = ((a1 * m2 * n2) + (a2 * m1 * n1)).rem_euclid(n1 * n2);
        a1 = solution;
        n1 *= n2;
    }

    solution
}

pub fn calculate(input: &str) {
    let mut lines = input.split('\n');
    let now = u64::from_str_radix(lines.next().unwrap(), 10).unwrap();
    let routes = lines.next().unwrap();

    let wtime = waitingtime(now, routes);
    println!(
        "Line {}, waiting time {}, answer: {}",
        wtime.0,
        wtime.1,
        wtime.0 * wtime.1
    );

    let dtime = departtime(routes);
    println!("Time where buses depart perfectly: {}", dtime);
}
