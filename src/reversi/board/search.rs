use super::Board;

impl Board {
    fn recursive(&self, focus: (usize, usize), side: char, count: usize, direct: &str) -> ((usize, usize), usize) {
        let mut result: ((usize, usize), usize) = (focus, count);

        let x = focus.0;
        let y = focus.1;

        let mut next_x = focus.0;
        let mut next_y = focus.1;

        if direct == "u" && !(y == 0) {
            next_y = next_y - 1;
        } else if direct == "ur" && !(x == 7 || y == 0) {
            next_x = next_x + 1;
            next_y = next_y - 1;
        } else if direct == "r" && !(x == 7) {
            next_x = next_x + 1;
        } else if direct == "dr" && !(x == 7 || y == 7) {
            next_x = next_x + 1;
            next_y = next_y + 1;
        } else if direct == "d" && !(y == 7) {
            next_y = next_y + 1;
        } else if direct == "dl" && !(x == 0 || y == 7) {
            next_x = next_x - 1;
            next_y = next_y + 1;
        } else if direct == "l" && !(x == 0) {
            next_x = next_x - 1;
        } else if direct == "ul" && !(x == 0 || y == 0) {
            next_x = next_x - 1;
            next_y = next_y - 1;
        }

        if self.board[next_x][next_y] == side {
            result = ((next_x, next_y), count);
        } else if self.board[next_x][next_y] == 'e' {
            result = ((next_x, next_y), 0);
        } else if self.board[next_x][next_y] != side {
            result = self.recursive((next_x, next_y), side, count + 1, direct);
        }

        return result;
    }

    fn near_check(&self, focus: (usize, usize), side: char) -> Vec<((usize, usize), &str)> {
        let mut result: Vec<((usize, usize), &str)> = vec![];
        let mut indexes: Vec<bool> = vec![];

        // 周囲八マスの探索をして探索できるところを決定する
        // (-1, -1) ( 0, -1) (+1, -1)
        // (-1,  0) ( 0,  0) (+1,  0)
        // (-1, +1) ( 0, +1) (+1, +1)

        // エッジケースへの対応
        if focus.0 == 7 && focus.1 == 0 {
            result.push(((focus.0, focus.1 + 1), "d"));
            result.push(((focus.0 - 1, focus.1 + 1), "dl"));
            result.push(((focus.0 - 1, focus.1), "l"));
        }
        else if focus.0 == 7 && focus.1 == 7 {
            result.push(((focus.0, focus.1 - 1), "u"));
            result.push(((focus.0 - 1, focus.1), "l"));
            result.push(((focus.0 - 1, focus.1 - 1), "ul"));
        } 
        else if focus.0 == 0 && focus.1 == 7 {
            result.push(((focus.0, focus.1 - 1), "u"));
            result.push(((focus.0 + 1, focus.1 - 1), "ur"));
            result.push(((focus.0 + 1, focus.1), "r"));
        } 
        else if focus.0 == 0 && focus.1 == 0 {
            result.push(((focus.0 + 1, focus.1), "l"));
            result.push(((focus.0 + 1, focus.1 + 1), "dl"));
            result.push(((focus.0, focus.1 + 1), "d"));
        } 
        else if focus.1 == 0 {
            result.push(((focus.0 + 1, focus.1), "r"));
            result.push(((focus.0 + 1, focus.1 + 1), "dr"));
            result.push(((focus.0, focus.1 + 1), "d"));
            result.push(((focus.0 - 1, focus.1 + 1), "dl"));
            result.push(((focus.0 - 1, focus.1), "l"));
        } 
        else if focus.0 == 7 {
            result.push(((focus.0, focus.1 - 1), "u"));
            result.push(((focus.0, focus.1 + 1), "d"));
            result.push(((focus.0 - 1, focus.1 + 1), "dl"));
            result.push(((focus.0 - 1, focus.1), "l"));
            result.push(((focus.0 - 1, focus.1 - 1), "ul"));
        } 
        else if focus.1 == 7 {
            result.push(((focus.0, focus.1 - 1), "u"));
            result.push(((focus.0 + 1, focus.1 - 1), "ur"));
            result.push(((focus.0 + 1, focus.1), "r"));
            result.push(((focus.0 - 1, focus.1), "l"));
            result.push(((focus.0 - 1, focus.1 - 1), "ul"));
        } 
        else if focus.0 == 0 {
            result.push(((focus.0, focus.1 - 1), "u"));
            result.push(((focus.0 + 1, focus.1 - 1), "ur"));
            result.push(((focus.0 + 1, focus.1), "r"));
            result.push(((focus.0 + 1, focus.1 + 1), "dr"));
            result.push(((focus.0, focus.1 + 1), "d"));
        } 
        else {
            result.push(((focus.0, focus.1 - 1), "u"));
            result.push(((focus.0 + 1, focus.1 - 1), "ur"));
            result.push(((focus.0 + 1, focus.1), "r"));
            result.push(((focus.0 + 1, focus.1 + 1), "dr"));
            result.push(((focus.0, focus.1 + 1), "d"));
            result.push(((focus.0 - 1, focus.1 + 1), "dl"));
            result.push(((focus.0 - 1, focus.1), "l"));
            result.push(((focus.0 - 1, focus.1 - 1), "ul"));
        }

        for i in 0..result.len() {
            if self.board[result[i].0.0][result[i].0.1] == 'e' {
                indexes.push(false);
            } else if self.board[result[i].0.0][result[i].0.1] == side {
                indexes.push(false);
            } else {
                indexes.push(true);
            }
        }

        let mut iter = indexes.iter();
        result.retain(|_| *iter.next().unwrap());

        return result;
    }

    fn target(&self, focus: (usize, usize), side: char) -> Vec<((usize, usize), usize)> {
        let mut result: Vec<((usize, usize), usize)> = vec![];
        let mut indexes: Vec<bool> = vec![];

        let directions: Vec<((usize, usize), &str)> = self.near_check(focus, side);

        for i in 0..directions.len() {
            result.push(self.recursive(directions[i].0, side, 1, directions[i].1));
        }

        for i in 0..result.len() {
            if result[i].1 == 0 {
                indexes.push(false);
            } else {
                indexes.push(true);
            }
        }

        let mut iter = indexes.iter();
        result.retain(|_| *iter.next().unwrap());

        return result;
    }

    pub fn search(&self, side: char) -> Vec<((usize, usize), (usize, usize), usize)> {

        let mut points: Vec<((usize, usize), (usize, usize), usize)> = vec![];

        for i in 0..8 {
            for j in 0..8 {
                if self.board[i][j] == 'e' {
                    let focus: (usize, usize) = (i, j);
                    let target: Vec<((usize, usize), usize)> = self.target(focus, side);

                    if !target.is_empty() {
                        for i in 0..target.len() {
                            points.push((focus, (target[i].0), target[i].1))
                        }
                    }
                }
            }
        }

        return points;
    }
}
