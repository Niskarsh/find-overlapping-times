#![allow(non_snake_case)]

pub fn findOverlappingTimes(mut timeset1: Vec<Vec<i32>>, mut timeset2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut overlappingTimes: Vec<Vec<i32>> = vec![];
    timeset1.sort_by(|a, b| a[0].cmp(&b[0]));
    timeset2.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut counter1 = 0;
    let mut counter2 = 0;
    while counter1 < timeset1.len() && counter2 < timeset2.len() {
        let start = if timeset1[counter1][0] > timeset2[counter2][0] { timeset1[counter1][0] } else { timeset2[counter2][0] };
        let end = if timeset1[counter1][1] < timeset2[counter2][1] { timeset1[counter1][1] } else { timeset2[counter2][1] };
        if timeset1[counter1][1] < timeset2[counter2][1] {
            counter1 += 1;
        } else {
            counter2 += 1;
        }
        if start < end {
            overlappingTimes.push(vec![start, end]);
        }
    }
    overlappingTimes
}