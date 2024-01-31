use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn solve(board: &str, strategy: &str) {
    if strategy == "DFS" {
        alert(&format!("{}, {}!", board, strategy));
    }
}

#[cfg(test)]
mod tests;