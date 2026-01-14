fn final_prices(prices: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();
    for (i, num1) in prices.iter().enumerate() {
        let mut inserted = false;
        for num2 in prices.iter().skip(i+1) {
            if *num2 < *num1 {
                res.push(*num1 - *num2);
                inserted = true;
                break;
            }
        }
        if !inserted {
            res.push(*num1);
        }
    }

    res
}

pub fn main() {
    let prices = vec![8,4,6,2,3];
    println!("{:?}", final_prices(prices));
}
