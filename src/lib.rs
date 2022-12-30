mod utils;

use wasm_bindgen::prelude::*;
use sha2::{Sha256, Digest}; 
use base16;                 

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, rust-web-assembly!");
}

#[wasm_bindgen]
pub fn calc_pi(count: u64) -> f64 { 
    let mut i = 1_u64; 
    let mut value = 1.0_f64; 
    while i <= count { 
        let index = 2 * i - 1;
        value = value
            - 1.0 / ((2 * index + 1) as f64)
            + 1.0 / ((2 * (index + 1) + 1) as f64);
        i += 1;
    }
    return value * 4.0; 
}


// SHA-2の計算処理 ...（3）
#[wasm_bindgen]
pub fn sha2(input: String) -> String {
    // SHA-2処理オブジェクトを生成 ...（4）
    let mut hasher = Sha256::new();
    // ハッシュ計算 ...（5）
    hasher.update(input);
    // 計算結果を出力 ...（6）
    let hash = hasher.finalize();
    // 計算結果を16進文字列に変換 ...（7）
    let encoded = base16::encode_lower(&hash);
    // 16進文字列を返却
    return encoded;
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn get_greet_text(name: &str) -> String {
    let greet_text = format!("こんにちは、{}さん!", name);
    log(&greet_text);  
    return greet_text;
}



