use super::{choice::Choice, point::Point, Board};

impl Board {
    fn recursive(&self, focus: Point, side: char, count: usize, direct: &str) -> (Point, usize) {
        let mut result: (Point, usize) = (focus.copy(), count);

        let x = focus.copy().x;
        let y = focus.copy().y;

        let mut next_x = focus.copy().x;
        let mut next_y = focus.copy().y;

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
            result = (Point::new(next_x, next_y), count);
        } else if self.board[next_x][next_y] == 'e' {
            result = (Point::new(next_x, next_y), 0);
        } else if self.board[next_x][next_y] != side {
            result = self.recursive(Point::new(next_x, next_y), side, count + 1, direct);
        }

        return result;
    }

    fn near_check(&self, focus: Point, side: char) -> Vec<(Point, &str)> {
        let mut result: Vec<(Point, &str)> = vec![];
        let mut indexes: Vec<bool> = vec![];

        // 周囲八マスの探索をして探索できるところを決定する
        // (-1, -1) ( 0, -1) (+1, -1)
        // (-1,  0) ( 0,  0) (+1,  0)
        // (-1, +1) ( 0, +1) (+1, +1)

        // エッジケースへの対応
        if focus.x == 7 && focus.y == 0 {
            result.push((Point::new(focus.x, focus.y + 1), "d"));
            result.push((Point::new(focus.x - 1, focus.y + 1), "dl"));
            result.push((Point::new(focus.x - 1, focus.y), "l"));
        } else if focus.x == 7 && focus.y == 7 {
            result.push((Point::new(focus.x, focus.y - 1), "u"));
            result.push((Point::new(focus.x - 1, focus.y), "l"));
            result.push((Point::new(focus.x - 1, focus.y - 1), "ul"));
        } else if focus.x == 0 && focus.y == 7 {
            result.push((Point::new(focus.x, focus.y - 1), "u"));
            result.push((Point::new(focus.x + 1, focus.y - 1), "ur"));
            result.push((Point::new(focus.x + 1, focus.y), "r"));
        } else if focus.x == 0 && focus.y == 0 {
            result.push((Point::new(focus.x + 1, focus.y), "l"));
            result.push((Point::new(focus.x + 1, focus.y + 1), "dl"));
            result.push((Point::new(focus.x, focus.y + 1), "d"));
        } else if focus.y == 0 {
            result.push((Point::new(focus.x + 1, focus.y), "r"));
            result.push((Point::new(focus.x + 1, focus.y + 1), "dr"));
            result.push((Point::new(focus.x, focus.y + 1), "d"));
            result.push((Point::new(focus.x - 1, focus.y + 1), "dl"));
            result.push((Point::new(focus.x - 1, focus.y), "l"));
        } else if focus.x == 7 {
            result.push((Point::new(focus.x, focus.y - 1), "u"));
            result.push((Point::new(focus.x, focus.y + 1), "d"));
            result.push((Point::new(focus.x - 1, focus.y + 1), "dl"));
            result.push((Point::new(focus.x - 1, focus.y), "l"));
            result.push((Point::new(focus.x - 1, focus.y - 1), "ul"));
        } else if focus.y == 7 {
            result.push((Point::new(focus.x, focus.y - 1), "u"));
            result.push((Point::new(focus.x + 1, focus.y - 1), "ur"));
            result.push((Point::new(focus.x + 1, focus.y), "r"));
            result.push((Point::new(focus.x - 1, focus.y), "l"));
            result.push((Point::new(focus.x - 1, focus.y - 1), "ul"));
        } else if focus.x == 0 {
            result.push((Point::new(focus.x, focus.y - 1), "u"));
            result.push((Point::new(focus.x + 1, focus.y - 1), "ur"));
            result.push((Point::new(focus.x + 1, focus.y), "r"));
            result.push((Point::new(focus.x + 1, focus.y + 1), "dr"));
            result.push((Point::new(focus.x, focus.y + 1), "d"));
        } else {
            result.push((Point::new(focus.x, focus.y - 1), "u"));
            result.push((Point::new(focus.x + 1, focus.y - 1), "ur"));
            result.push((Point::new(focus.x + 1, focus.y), "r"));
            result.push((Point::new(focus.x + 1, focus.y + 1), "dr"));
            result.push((Point::new(focus.x, focus.y + 1), "d"));
            result.push((Point::new(focus.x - 1, focus.y + 1), "dl"));
            result.push((Point::new(focus.x - 1, focus.y), "l"));
            result.push((Point::new(focus.x - 1, focus.y - 1), "ul"));
        }

        for i in 0..result.len() {
            if self.board[result[i].0.x][result[i].0.y] == 'e' {
                indexes.push(false);
            } else if self.board[result[i].0.x][result[i].0.y] == side {
                indexes.push(false);
            } else {
                indexes.push(true);
            }
        }

        let mut iter = indexes.iter();
        result.retain(|_| *iter.next().unwrap());

        return result;
    }

    fn target(&self, focus: Point, side: char) -> (Vec<Point>, usize) {
        let mut result: (Vec<Point>, usize) = (vec![], 0);

        let directions: Vec<(Point, &str)> = self.near_check(focus, side);

        for i in 0..directions.len() {
            let focus = directions[i].0.copy();
            let direct = directions[i].1;

            let tmp: (Point, usize) = self.recursive(focus, side, 1, direct);

            if tmp.1 != 0 {
                result.0.push(tmp.0);
                result.1 = result.1 + tmp.1;
            }
        }

        return result;
    }

    pub fn search(&self, side: char) -> Vec<Choice> {
        let mut points: Vec<Choice> = vec![];

        for i in 0..8 {
            for j in 0..8 {
                if self.board[i][j] == 'e' {
                    let focus: Point = Point::new(i, j);
                    let target: (Vec<Point>, usize) = self.target(focus, side);

                    if !target.0.is_empty() {
                        let focus: Point = Point::new(i, j);
                        let mut choice: Choice = Choice::new(focus);
                        choice.targets = target.0;
                        choice.count = target.1;
                        points.push(choice)
                    }
                }
            }
        }

        return points;
    }
}
