use crate::reversi::board::{choice::Choice, point::Point};

use super::Player;

impl Player {
    fn recursive_search(&self, index: usize, choice: Choice, priority: usize) -> usize {
        return 0;
    }

    fn corner(&self) -> Vec<usize> {
        let corner: Vec<Point> = vec![Point::new(0, 0), Point::new(7, 0), Point::new(7, 7), Point::new(0, 7)];

        let choices: Vec<Choice> = self.board.search(self.side);
        let mut indexes: Vec<usize> = vec![];

        for i in 0..choices.len() {
            for j in 0..corner.len() {
                if choices[i].focus.equal(corner[j].copy()) {
                    indexes.push(i);
                }
            }
        }

        return indexes;
    }

    pub fn select(&self) -> Point {
        let result: Point = Point::new(0, 0);

        let choices: Vec<Choice> = self.board.search(self.side);

        // 優先度
        let mut priorities: Vec<usize> = vec![];

        // - 角に打てるかチェック
        let indexes: Vec<usize> = self.corner();

        if indexes.len() == 1 {
            let tmp: usize = indexes[0];
            return choices[tmp].copy().focus;
        } else if indexes.len() > 4 {
            for i in 0..choices.len() {
                priorities.push(self.recursive_search(i, choices[i].copy(), 0))
            }
        } else {
            for i in 0..indexes.len() {
                priorities.push(self.recursive_search(indexes[i], choices[indexes[i]].copy(), 0))
            }
        }
        //   - 1つなら打つ
        //   - 撃てる箇所が複数なら選択肢を絞って次へ
        //   - 角が撃てないならそのまま次へ
        // - 取った後に相手が打てる場所の把握
        //    * 相手が角を取れるようになったらダメ
        //    * 相手が打てる一が最も少なくなるようにする
        // - 同じ位置から取れる駒の数

        return result;
    }
}
