// from : 起点柱
// via：中间柱
// to：目标柱
// n: 盘子数量
// moves: 记录步骤用的列表
pub fn hanoi(
    n: i32, 
    from : char, 
    via: char, 
    to: char,
    moves: &mut Vec<(char, char)>,
) {
    if n > 0 {
        match n {
            1 => moves.push((from, to)),
            n => {
                // 把n-1个盘子从A移动到B
                hanoi(n-1, from, to, via, moves);
                // 把最后一个盘子从A移动到C
                hanoi(1, from, via, to, moves);
                // 把n-1个盘子从B移动到C
                hanoi(n-1, via, from, to, moves);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hanoi_1() {
        let mut moves = Vec::new();
        hanoi(1, 'A', 'B', 'C', &mut moves);
        assert_eq!(vec![('A', 'C')], moves);
    }

    #[test]
    fn test_hanoi_3() {
        let mut moves = Vec::new();
        hanoi(3, 'A', 'B', 'C', &mut moves);
        assert_eq!(
            vec![('A', 'C'), ('A', 'B'), ('C', 'B'), ('A', 'C'), ('B', 'A'), ('B', 'C'), ('A', 'C')],
            moves
        );
    }
}