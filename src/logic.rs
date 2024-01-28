use crate::attachment::attachment;
use crate::reversi::player::Player;

pub fn logic(input: String) -> String {
    let data: ([[char; 8]; 8], char) = attachment(input);

    // Playerを起動
    // Palyer内のsideにデータを入力
    // Player内のboardにデータを入力
    let player: Player = Player::new(data.1, data.0);

    // Player内のselectを実行し結果を受け取る
    let result: (usize, usize) = player.random();

    // 受け取ったデータをjsonへ変換
    let json: String = "{x: ".to_owned() + &result.0.to_string() + ", y: " + &result.1.to_string() + " }";

    return json;
}
