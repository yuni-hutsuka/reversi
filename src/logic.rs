use crate::attachment::attachment;
use crate::reversi::board::Board;

pub fn logic(input: String) -> String {
    let result: String = "".to_owned();

    let data: ([[char; 8]; 8], char) = attachment(input);

    // Playerを起動
    // Palyer内のsideにデータを入力
    // Player内のboardにデータを入力
    // Player内のselectを実行し結果を受け取る
    // 受け取ったデータをjsonへ変換

    return result;
}
