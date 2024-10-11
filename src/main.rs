use std::io;
mod huffman;

fn main() {
    let mut input: String = String::new();
    
    loop {
        input.clear();
        println!("\nEnter the string you want to encode ([q] to quit): ");
        io::stdin().read_line(&mut input).expect("[ERRO] failed to read line");
    
        if input.trim().to_lowercase() == "q" { 
            println!("\nSee you later!");
            break; 
        }        

        let (encoded_text, codewords) = huffman::text_encoder(&input.trim());
        let splitter = "=".to_string().repeat(75);

        println!("\n{}", splitter);
        println!("\nEncoded text:\n{}", encoded_text);
        println!("\nCodewords:\n{:?}", codewords);
        println!("\n{}", splitter);

    }
}