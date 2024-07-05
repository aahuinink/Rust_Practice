use std::collections::HashMap;
use std::thread;

fn main() {
    let list = vec![1,2,3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}")).join().unwrap();
}

fn median(vec: &Vec<i32>) -> Result<i32, &str> {
    let mut sorted: Vec<i32> = vec.clone(); // make mutable deep copy of vec
    sorted.sort(); // sort the vec
    let length: usize = sorted.len();
    if length == 0 {
        return Err("Vector Length is 0!")
    }
    if (length & 0x1) == 0x1 {          // if odd
        Ok(sorted[length>>1])       // index half the length
    } else {        // if even
        Ok((sorted[length >> 1] + sorted[length >> 1 - 1]) >> 1)
    }
}

fn mode(vec: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for i in vec.iter()
    {
        map.entry(*i).and_modify(|counter| {*counter += 1}).or_insert(1);
    }
    let mut count: i32 = 0;
    let mut mode: i32 = 0;
    for (key, val) in map.iter() {
        if *val > count {
            mode = *key;
            count = *val;
        }
    }
    mode
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn odd_count() {
        match median(&vec![1,4,2,7,6]) {
            Ok(med) => assert_eq!(4,med),
            Err(error) => assert_eq!("Vector Length is 0!", error)
        }
    }

    #[test]
    fn ones () {
        let mode: i32 = mode(&vec![1,1,1,1,4,4,4,5,7]);
        assert_eq!(1,mode)
    }
}
