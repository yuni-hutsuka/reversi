use crate::attachment::attachment;
use crate::reversi::board::point::Point;
use crate::reversi::player::Player;

pub fn logic(input: String) -> String {
    let data: ([[char; 8]; 8], char) = attachment(input);

    // Playerを起動
    // Palyer内のsideにデータを入力
    // Player内のboardにデータを入力
    let player: Player = Player::new(data.1, data.0);

    // Player内のselectを実行し結果を受け取る
    // let result: Point = player.random();
    let result: Point = player.select();

    // 受け取ったデータをjsonへ変換
    let json: String =
        "{x: ".to_owned() + &result.x.to_string() + ", y: " + &result.y.to_string() + " }";

    return json;
}
