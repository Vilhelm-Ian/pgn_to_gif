use image::{open};
use pgnparse::parser::*;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;


const PIECES: [&str; 12] = ["p", "n", "b", "r", "k", "q", "P", "N", "B", "R", "K", "Q"];
const OFF_SET: i64 = 7;

fn main() {
    let background = open("../assets/board.png").unwrap().into_rgba8();
    let mut pieces_images = HashMap::new();
    for (_index, piece) in PIECES.iter().enumerate() {
        let path = format!("../assets/{piece}.png");
        let image = open(path).unwrap().into_rgba8();
        pieces_images.insert(*piece, image);
    }
    let pgn = "[Date '1992.11.04']
[Round '29']
[White 'Fischer, Robert J.']
[Black 'Spassky, Boris V.']
[Result '1/2-1/2']

1. e4 e5 2. Nf3 Nc6 3. Bb5 a6 {This opening is called the Ruy Lopez.}
4. Ba4 Nf6 5. O-O Be7 6. Re1 b5 7. Bb3 d6 8. c3 O-O 9. h3 Nb8 10. d4 Nbd7
11. c4 c6 12. cxb5 axb5 13. Nc3 Bb7 14. Bg5 b4 15. Nb1 h6 16. Bh4 c5 17. dxe5
Nxe4 18. Bxe7 Qxe7 19. exd6 Qf6 20. Nbd2 Nxd6 21. Nc4 Nxc4 22. Bxc4 Nb6
23. Ne5 Rae8 24. Bxf7+ Rxf7 25. Nxf7 Rxe1+ 26. Qxe1 Kxf7 27. Qe3 Qg5 28. Qxg5
hxg5 29. b3 Ke6 30. a3 Kd6 31. axb4 cxb4 32. Ra5 Nd5 33. f3 Bc8 34. Kf2 Bf5
35. Ra7 g6 36. Ra6+ Kc5 37. Ke1 Nf4 38. g3 Nxh3 39. Kd2 Kb5 40. Rd6 Kc5 41. Ra6
Nf2 42. g4 Bd3 43. Re6 1/2-1/2";

    let pgn_struc = parse_pgn_to_rust_struct(pgn);
    let moves = get_fens(pgn_struc);
    let mut result = vec![background.clone(); moves.len()];
    let buffer = File::create("foo.gif").unwrap();
    let mut map = vec![];
    let mut gif = image::codecs::gif::GifEncoder::new(buffer);
    moves
        .par_iter()
        .enumerate()
        .map(|(index, _move)| {
            let mut image = background.clone();
            fen_to_image(_move, &pieces_images, &mut image);
            (index, image)
        })
        .collect_into_vec(&mut map);
    for (index, _move) in map {
        result[index] = _move
    }
    let mut frames = vec![];
    for image in result {
        let frame = image::Frame::new(image);
        frames.push(frame)
    }
    match gif.encode_frames(frames) {
        Ok(_) => println!("gif encoded"),
        Err(err) => println!("problem encoding gif {err:?}"),
    };
    println!("Hello, world!");
}

fn fen_to_image(
    fen: &str,
    images_map: &HashMap<&str, image::RgbaImage>,
    background: &mut image::RgbaImage,
) {
    let fen: Vec<&str> = fen.split('/').collect();
    for (y, line) in fen.iter().enumerate() {
        //        let line: Vec<&str> = line.split("").collect();
        let mut x = -1;
        for value in line.chars() {
            match value.to_digit(10) {
                Some(number) => {
                    x += number as i32;
                    continue;
                }
                None => x += 1,
            }
            let image = images_map.get(String::from(value).as_str()).unwrap();
            image::imageops::overlay(
                background,
                image,
                (300 / 8) * x as i64 + OFF_SET,
                (300 / 8) * y as i64 + OFF_SET,
            );
        }
    }
}

fn get_fens(data: PgnInfo) -> Vec<String> {
    let mut result = vec![];
    for _move in data.moves {
        let fen: Vec<&str> = _move.fen_after.split(' ').collect();
        result.push(String::from(fen[0]))
    }
    result
}
