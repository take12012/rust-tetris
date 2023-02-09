use std::{thread, time};
use getch_rs::{Getch, Key};

// テトリミノの種類
#[derive(Clone, Copy)]
enum MinoKind {
    I,
    O,
    S,
    Z,
    J,
    L,
    T
}

// テトリミノの形状
const MINOS: [[[usize; 4]; 4]; 7] = [
    // Iミノ
    [
        [0,0,0,0],
        [0,0,0,0],
        [1,1,1,1],
        [0,0,0,0],
    ],
    // Oミノ
    [
        [0,0,0,0],
        [0,1,1,0],
        [0,1,1,0],
        [0,0,0,0],
    ],
    // Sミノ
    [
        [0,0,0,0],
        [0,1,1,0],
        [1,1,0,0],
        [0,0,0,0],
    ],
    // Zミノ
    [
        [0,0,0,0],
        [1,1,0,0],
        [0,1,1,0],
        [0,0,0,0],
    ],
    // Jミノ
    [
        [0,0,0,0],
        [1,0,0,0],
        [1,1,1,0],
        [0,0,0,0],
    ],
    // Lミノ
    [
        [0,0,0,0],
        [0,0,1,0],
        [1,1,1,0],
        [0,0,0,0],
    ],
    // Tミノ
    [
        [0,0,0,0],
        [0,1,0,0],
        [1,1,1,0],
        [0,0,0,0],
    ],
];

 // テトリミノがフィールドに衝突する場合は`ture`を返す
 fn is_collision(field: &[[usize;12]], pos: &Position, mino: MinoKind) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if field[y+pos.y][x+pos.x] & MINOS[mino as usize][y][x] == 1 {
                return true;
            }
        }
    }
    false
}

struct Position {
    x: usize,
    y: usize,
}

fn main() {

    let field = [
        [1,1,1,0,0,0,0,0,0,1,1,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,1,1,1,1,1,1,1,1,1,1,1],
    ];
    
    // 初期位置
    let mut pos = Position { x: 4, y: 0 };
    let g = Getch::new();
    
    // 画面クリア
    println!("\x1b[2J\x1b[H\x1b[?25l");

    // 30マス分落下させてみる
    loop {
        // 描画用フィールドの生成
        let mut field_buf = field;
        // 自然落下
        let new_pos = Position {
            x: pos.x,
            y: pos.y + 1,
        };
        if !is_collision(&field, &new_pos, MinoKind::I) {
            // posの座標を更新
            pos = new_pos;
        }
        // 描画用フィールドにテトリミノの情報を書き込む
        for y in 0..4 {
            for x in 0..4 {
                field_buf[y+pos.y][x+pos.x] |= MINOS[MinoKind::I as usize][y][x];
            }
        }
        // フィールドを描画
        println!("\x1b[H");  // カーソルを先頭に移動
        for y in 0..22 {
            for x in 0..12 {
                if field_buf[y][x] == 1 {
                    print!("[]");
                } else {
                    print!(" .");
                }
            }
            println!();
        }

        // 1秒間スリーブする
        thread::sleep(time::Duration::from_millis(1000));
        // キー入力待ち
        match g.getch() {
            Ok(Key::Left) => {
                let new_pos = Position {
                    x: pos.x - 1,
                    y: pos.y,
                };
                if !is_collision(&field, &new_pos, MinoKind::I) {
                    // posの座標を更新
                    pos = new_pos;
                }
            }
            Ok(Key::Down) => {
                let new_pos = Position {
                    x: pos.x,
                    y: pos.y + 1,
                };
                if !is_collision(&field, &new_pos, MinoKind::I) {
                    // posの座標を更新
                    pos = new_pos;
                }
            }
            Ok(Key::Right) => {
                let new_pos = Position {
                    x: pos.x + 1,
                    y: pos.y,
                };
                if !is_collision(&field, &new_pos, MinoKind::I) {
                    // posの座標を更新
                    pos = new_pos;
                }
            }
            Ok(Key::Char('q')) => break,
            _ => (),  // 何もしない
       }
    }

    // カーソルを再表示
    println!("\x1b[?25h");
}