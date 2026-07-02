fn count_coins(sum: usize, coins: &Vec<usize>, mut coin_index: usize) -> usize {
    let length = coins.len();
    let mut res = 0;
    if length <= coin_index {
        // I don't think we should get here, but if we happen to then
        // we can handle this in the future
        return 0;
    }

    if length == 1 {
        if coins[coin_index] == sum {
            return 1;
        } else {
            return 0;
        }
    }

    if sum == coins[coin_index] {
        return 1;
    }

    if sum > coins[coin_index] {
        res += count_coins(sum - coins[coin_index], coins, coin_index);
        println!("sum: {}, coin: {}", sum, coins[coin_index]);
    }
    coin_index += 1;
    res += count_coins(sum, coins, coin_index);
    return res;
}

fn count_coins_memoized(
    sum: usize,
    coins: &Vec<usize>,
    mut coin_index: usize,
    mem: Option<Vec<usize>>,
) -> usize {
    let empty = vec![0; sum + 1];
    let mut memo = mem.unwrap_or(empty);
    let length = coins.len();
    let mut res = 0;
    if length <= coin_index {
        // I don't think we should get here, but if we happen to then
        // we can handle this in the future
        return 0;
    }

    if length == 1 {
        if coins[coin_index] == sum {
            return 1;
        } else {
            return 0;
        }
    }

    if sum == coins[coin_index] {
        return 1;
    }

    if memo[sum] != 0 {
        return memo[sum];
    }

    if sum > coins[coin_index] {
        res += count_coins_memoized(
            sum - coins[coin_index],
            coins,
            coin_index,
            Some(memo.to_vec()),
        );
        println!("sum: {}, coin: {}", sum, coins[coin_index]);
    }
    coin_index += 1;
    res += count_coins_memoized(sum, coins, coin_index, Some(memo.to_vec()));
    memo[sum] = res;
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // if the sum and only coin available are the same return 1
    fn count_coins_01() {
        let coins = vec![0];
        let count = count_coins(0, &coins, 0);
        let res = 1;
        assert_eq!(count, res);
    }

    #[test]
    // Simple case
    fn count_coins_02() {
        let coins = vec![2, 1];
        let count = count_coins(3, &coins, 0);
        // Should be [2,2,1], [2,1,1,1], [1,1,1,1,1]
        // Note all other combinations are already included in this
        let res = 2;
        assert_eq!(count, res);
    }

    #[test]
    // intermediate case
    fn count_coins_03() {
        let coins = vec![2, 1];
        let count = count_coins(5, &coins, 0);
        // Should be [2,2,1], [2,1,1,1], [1,1,1,1,1]
        // Note all other combinations are already included in this
        let res = 3;
        assert_eq!(count, res);
    }

    #[test]
    // if the sum and only coin available are the same return 1
    fn count_coins_04() {
        let coins = vec![0];
        let count = count_coins_memoized(0, &coins, 0, None);
        let res = 1;
        assert_eq!(count, res);
    }

    #[test]
    // Simple case
    fn count_coins_05() {
        let coins = vec![2, 1];
        let count = count_coins_memoized(3, &coins, 0, None);
        // Should be [2,2,1], [2,1,1,1], [1,1,1,1,1]
        // Note all other combinations are already included in this
        let res = 2;
        assert_eq!(count, res);
    }

    #[test]
    // intermediate case
    fn count_coins_06() {
        let coins = vec![2, 1];
        let count = count_coins_memoized(5, &coins, 0, None);
        // Should be [2,2,1], [2,1,1,1], [1,1,1,1,1]
        // Note all other combinations are already included in this
        let res = 3;
        assert_eq!(count, res);
    }
}

fn main() {
    println!("Hello, world!");
    let coins = vec![1, 2, 5, 10, 20, 50, 100, 200];
    let goal = 200;
    let res = count_coins_memoized(goal, &coins, 0, None);
    println!("{}", res);
}
