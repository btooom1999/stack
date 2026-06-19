fn dfs(
    num_arrows: i32,
    i: usize,
    alice_arrows: &Vec<i32>,
    bob_arrows: &mut usize,
    memo: &mut [usize],
) -> usize {
    if i == 12 {
        let mut bob = 0;
        for i in 0..12 {
            if *bob_arrows >> i & 1 == 1 {
                bob += i;
            }
        }

        memo[bob] = *bob_arrows;
        return bob;
    }

    let mut res = dfs(num_arrows, i+1, alice_arrows, bob_arrows, memo);
    if num_arrows-1-alice_arrows[i] >= 0 {
        *bob_arrows ^= 1 << i;
        res = res.max(dfs(num_arrows-alice_arrows[i]-1, i+1, alice_arrows, bob_arrows, memo));
        *bob_arrows ^= 1 << i;
    }

    res
}

fn maximum_bob_points(mut num_arrows: i32, alice_arrows: Vec<i32>) -> Vec<i32> {
    let mut memo = [0; 67];
    let max = dfs(num_arrows, 0, &alice_arrows, &mut 0, &mut memo);

    let mut res = vec![0; 12];
    for i in 0..12 {
        if memo[max] >> i & 1 == 1 {
            res[i] = alice_arrows[i] + 1;
            num_arrows -= alice_arrows[i]+1;
        }
    }

    res[11] += num_arrows;
    res
}

pub fn main() {
    let num_arrows = 100_000;
    let alice_arrows = [1,1,0,1,0,0,2,1,0,1,2,0].to_vec();
    println!("{:?}", maximum_bob_points(num_arrows, alice_arrows));
}
