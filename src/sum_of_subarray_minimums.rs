const MOD: i64 = 1_000_000_007;

fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
    let mut stack = Vec::<(i64, i64)>::new();
    let mut summed_stack = 0;
    let mut sum = 0;

    for num in &arr {
        let num = *num as i64;
        let mut count = 1;
        while let Some(&(v, c)) = stack.last() {
            if v < num {
                break;
            }

            count += c;
            summed_stack -= v * c;
            stack.pop();
        }

        stack.push((num, count));
        sum += summed_stack + (num * (count - 1)) + num;
        summed_stack += num * count;
    }

    (sum % MOD) as i32
}

pub fn main() {
    // let arr = vec![3,1,2,4];
    let arr = vec![11,81,94,43,3];
    println!("{}", sum_subarray_mins(arr));
}
