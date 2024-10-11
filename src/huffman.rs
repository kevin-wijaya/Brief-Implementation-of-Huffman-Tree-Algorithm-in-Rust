// construct a node for the binary tree in huffman coding tree
pub struct Node {
    frequency: usize,
    label: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

// calculate the frequency each character
pub fn get_frequencies(input: &str) -> Vec<(String, usize)> {
    let mut frequencies: Vec<(String, usize)> = Vec::new();

    for character in input.chars() {
        let position: Option<usize> = frequencies.iter()
            .position(|&(ref character_, _)| character_ == &character.to_string()
        );

        if position != None {
            let index = position.unwrap();
            frequencies[index].1 += 1;
        } else {
            frequencies.push((character.to_string(), 1));
        }
    }

    frequencies
}

// build the huffman tree for encoding and decoding
pub fn build_tree(frequencies: &[(String, usize)]) -> Option<Box<Node>> {
    let mut nodes: Vec<Node> = frequencies.iter()
        .map(|(character, frequency)| Node {
            frequency: *frequency,
            label: character.clone(),
            left: None,
            right: None,
        })
        .collect();

    let mut inner_node_counter: usize = 1;
    while nodes.len() > 1 {
        nodes.sort_by(|a, b| a.frequency.cmp(&b.frequency));

        let left: Node = nodes.remove(0);
        let right: Node = nodes.remove(0);

        let inner_node: Node = Node {
            frequency: left.frequency + right.frequency,
            label: format!("in{}", inner_node_counter),
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        };

        nodes.push(inner_node);
        inner_node_counter += 1;
    }

    nodes.pop().map(Box::new)
}

// generate codewords for each character according to huffman tree
pub fn generate_codewords(node: &Box<Node>, prefix: &str, codewords: &mut Vec<(String, String)>) {
    if node.left.is_none() && node.right.is_none() {
        codewords.push((node.label.clone(), prefix.to_string()));
    } else {
        if let Some(left) = &node.left {
            generate_codewords(
                &left, 
                &format!("{}0", prefix), 
                codewords
            );
        }
        if let Some(right) = &node.right {
            generate_codewords(
                &right, 
                &format!("{}1", prefix), 
                codewords
            );
        }
    }
}

// the encoder pipeline
pub fn text_encoder(input: &str) -> (String, Vec<(String, String)>) {
    let frequencies = get_frequencies(input);
    let huffman_tree: Box<Node> = build_tree(&frequencies).expect("test");
    let mut codewords: Vec<(String, String)> = Vec::new();

    generate_codewords(&huffman_tree, &String::new(), &mut codewords);

    let encoded_text: String = input.chars()
        .map(|character| {
            codewords.iter()
                .find(|&&(ref character_, _)| character_ == &character.to_string())
                .map(|&(_, ref codeword)| codeword.clone())
                .unwrap_or_else(String::new)
        })
        .collect();

    (encoded_text, codewords)
}