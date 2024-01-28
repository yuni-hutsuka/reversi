fn translate_board(before: &str) -> [[char; 8]; 8] {
    let mut result: [[char; 8]; 8]
        = [
            ['e', 'e', 'e', 'e', 'e', 'e', 'e', 'e'],
            ['e', 'e', 'e', 'e', 'e', 'e', 'e', 'e'],
            ['e', 'e', 'e', 'e', 'e', 'e', 'e', 'e'],
            ['e', 'e', 'e', 'e', 'e', 'e', 'e', 'e'],
            ['e', 'e', 'e', 'e', 'e', 'e', 'e', 'e'],
            ['e', 'e', 'e', 'e', 'e', 'e', 'e', 'e'],
            ['e', 'e', 'e', 'e', 'e', 'e', 'e', 'e'],
            ['e', 'e', 'e', 'e', 'e', 'e', 'e', 'e']
        ];

    let nums = before.chars().collect::<Vec<char>>();

    for i in 0..8 {
        for j in 0..8 {
            if nums[i * 8 + j] == '0' {
                result[i][j] = 'e';
            } else if nums[i * 8 + j] == '1' {
                result[i][j] = 'w';
            } else if nums[i * 8 + j] == '2' {
                result[i][j] = 'b';
            }
        }
    }

    return result;
}

fn translate_side(before: &str) -> char {
    let mut result: char = 'w';

    if before == "1" {
        result = 'w'
    } else if before == "2" {
        result = 'b'
    }

    return result;
}

pub fn attachment(input: String) -> ([[char; 8]; 8], char) {
    let before_board: &str = input.split(',').collect::<Vec<&str>>()[0];
    let before_side: &str = input.split(',').collect::<Vec<&str>>()[1];

    let board: [[char; 8]; 8] = translate_board(before_board);
    let side: char = translate_side(before_side);

    return (board, side);
}
