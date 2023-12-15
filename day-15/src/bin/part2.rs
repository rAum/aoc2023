fn main() {
    let input = include_str!("input");
    let result = solution(input);
    println!("Result: {}", result);
}

fn p(v: &[u8]) {
    print!("{}", std::str::from_utf8(v).unwrap());
}

fn solution(input: &str) -> usize {
    let bytes: Vec<u8> = input.trim_end_matches('\n').bytes().collect();
    let ranges = bytes.split(|&v| v == b',');

    let mut boxes: Vec<Vec<&[u8]>> = Vec::new();
    for _ in 0..256 {
        boxes.push(Vec::new());
    }
    for range in ranges {
        if range.ends_with(&[b'-']) {
            let key = &range[..range.len() - 1];
            let box_ = hash(key);
            let v = &mut boxes[box_];
            if let Some(index) = v.iter().position(|&v| v.starts_with(key)) {
                v.remove(index);
            }
        } else {
            let key = &range[..range.len() - 2];
            let box_ = hash(key);
            let v = &mut boxes[box_];
            if let Some(index) = v.iter().position(|&v| v.starts_with(key)) {
                v[index] = range;
            } else {
                v.push(range);
            }
        }
    }
    let mut result: usize = 0;

    for box_i in 0..boxes.len() {
        if boxes[box_i].is_empty() {
            continue;
        }
        for (lens_pos, &lens) in boxes[box_i].iter().enumerate() {
            let focal_l = *lens.last().unwrap() as usize - b'0' as usize;
            let box_n = box_i + 1;
            let slot = lens_pos + 1;
            let v = box_n * slot * focal_l;
            //p(lens); println!("(box {}) * {} (slot) * {} (focal) = {}", box_n, slot, focal_l, v);
            result += v;
        }
    }

    result
}

fn hash(range: &[u8]) -> usize {
    let mut hash = 0;

    for c in range.iter() {
        hash += *c as usize;
        hash *= 17;
        hash = hash % 256;
    }

    hash
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_solution() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let answer = 145;
        assert_eq!(solution(input), answer);
    }
}
