use super::Board;

impl Board {
    fn target(&self, focus: (usize, usize), side: char) -> Vec<((usize, usize), usize)> {
        return vec![];
    }
    pub fn search(&self, side: char) -> Vec<((usize, usize), (usize, usize), usize)> {

        let mut points: Vec<((usize, usize), (usize, usize), usize)> = vec![];

        for i in 0..8 {
            for j in 0..8 {
                if self.board[i][j] == 'e' {
                    // ターゲットの探索
                    // ターゲットポイントとそこまでの個数
                    let target: Vec<((usize, usize), usize)> = self.target((i, j), side);

                    if !target.is_empty() {
                        for i in 0..target.len() {
                            points.push(((i, j), (target[i].0), target[i].1))
                        }
                    }
                }
            }
        }

        return points;
    }
}
