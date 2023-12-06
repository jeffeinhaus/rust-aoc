pub fn wait_for_it(times: Vec<u64>, distances: Vec<u64>) {
    let mut result = 1;
    for (time, distance) in times.iter().zip(distances.iter()) {
        let mut record_times = 0;
        for speed in 0..*time {
            let traveled = (*time - speed) * speed;
            if traveled > *distance {
                record_times += 1;
            }
        }
        result *= record_times;
    }
    println!("Wait for it: {}", result);
}