fn maximum_gain(mut s: String, x: i32, y: i32) -> i32 {
    let (mut a, mut temp_b, mut point_a) = (0, 0, 0);
    let (mut b, mut temp_a, mut point_b) = (0, 0, 0);

    let mut res = 0;
    s.push('c');
    for c in s.chars() {
        if c == 'a' {
            a += 1;
            if b > 0 {
                b -= 1;
                point_b += y;
            } else {
                temp_a += 1;
            }
        } else if c == 'b' {
            b += 1;
            if a > 0 {
                a -= 1;
                point_a += x;
            } else {
                temp_b += 1;
            }
        } else {
            point_a += a.min(temp_b) * y;
            point_b += b.min(temp_a) * x;
            res += point_a.max(point_b);
            a = 0;
            b = 0;
            temp_a = 0;
            temp_b = 0;
            point_a = 0;
            point_b = 0;
        }
    }

    res
}

pub fn main() {
    let s = "abaabbaaxbbaabb".to_string();
    // let s = "abaabbaa".to_string();
    // let s = "bbaabb".to_string();
    let x = 4;
    let y = 5;
    println!("{}", maximum_gain(s, x, y));
}
