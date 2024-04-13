use find_overlapping_times::findOverlappingTimes;
fn main() {
    let secTimeSet1 = vec![vec![1, 3], vec![13, 15], vec![16,20]];
    let secTimeSet2 = vec![vec![2, 14]];
    let overlappingTimes = findOverlappingTimes(&secTimeSet1, &secTimeSet2);
    println!("Overlapping times: {:?}", overlappingTimes);
}
