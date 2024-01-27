pub static RULED_LINE_UL: char = '┌';
pub static RULED_LINE_UR: char = '┐';
pub static RULED_LINE_LL: char = '└';
pub static RULED_LINE_LR: char = '┘';
pub static RULED_LINE_V: char = '│';
pub static RULED_LINE_H: char = '─';

pub struct Board {
    pub board: [[i32; 8]; 8],
    pub side: bool,
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: [
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 1, 2, 0, 0, 0],
                [0, 0, 0, 2, 1, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
            ],
            side: true,
        }
    }

    pub fn print(self) {
        for i in 0..17 {
            for j in 0..17 {
                let tmp: char = match (i, j) {
                    (0, _) => match j {
                        0 => RULED_LINE_UL,
                        16 => RULED_LINE_UR,
                        _ => {
                            if j % 2 == 0 {
                                '┬'
                            } else {
                                RULED_LINE_H
                            }
                        }
                    },
                    (16, _) => match j {
                        0 => RULED_LINE_LL,
                        16 => RULED_LINE_LR,
                        _ => {
                            if j % 2 == 0 {
                                '┴'
                            } else {
                                RULED_LINE_H
                            }
                        }
                    },
                    (_, 0) => match i {
                        0 => RULED_LINE_UL,
                        16 => RULED_LINE_LL,
                        _ => {
                            if i % 2 == 0 {
                                '├'
                            } else {
                                RULED_LINE_V
                            }
                        }
                    },
                    (_, 16) => match i {
                        0 => RULED_LINE_UR,
                        16 => RULED_LINE_LR,
                        _ => {
                            if i % 2 == 0 {
                                '┤'
                            } else {
                                RULED_LINE_V
                            }
                        }
                    },
                    (_, _) => {
                        if i % 2 != 0 && j % 2 == 0 {
                            RULED_LINE_V
                        } else if i % 2 == 0 && j % 2 != 0 {
                            RULED_LINE_H
                        } else if i % 2 == 0 && j % 2 == 0 {
                            '┼'
                        } else {
                            let tmpi: usize = i / 2;
                            let tmpj: usize = j / 2;
                            if self.board[tmpi][tmpj] == 0 {
                                ' '
                            } else if self.board[tmpi][tmpj] == 2 {
                                '●'
                            } else {
                                '○'
                            }
                        }
                    }
                };
                print!("{}", tmp)
            }
            println!("");
        }
    }
}
