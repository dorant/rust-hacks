use std::fs::File;
use std::io::{self, Read, Write};

fn slice_to_string(slice: &[u32]) -> String {
    slice
        .iter()
        .map(|value| value.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn write_to_file(content: &str, filename: &str) -> io::Result<()> {
    let mut f = File::create(filename)?;
    f.write_all(content.as_bytes())
}

fn save_data(filename: &str, first: &Vec<u32>, second: &Vec<u32>) -> io::Result<()> {
    let s_first = slice_to_string(&first);
    let s_second = slice_to_string(&second);

    write_to_file(&format!("{}\n{}", s_first, s_second), &filename)
}

fn read_from_file(filename: &str) -> io::Result<String> {
    let mut f = File::open(filename)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

fn line_to_slice(line: &str) -> Vec<u32> {
    let res = line
        .split(" ")
        .filter_map(|num| num.parse::<u32>().ok())
        .collect();
    println!("line:{:?} res:{:?}", line, res);
    res
}

fn load_data(filename: &str) -> Option<(Vec<u32>, Vec<u32>)> {
    if let Ok(content) = read_from_file(filename) {
        let mut lines = content
            .splitn(3, "\n")
            .map(|line| line_to_slice(line))
            .collect::<Vec<_>>();
        if lines.len() == 2 {
            let (second_row, first_row) = (lines.pop().unwrap(), lines.pop().unwrap());
            Some((first_row, second_row))
        } else {
            None
        }
    } else {
        None
    }
}

fn main() {
    let filename = "./test.txt";

    let data_first = vec![1, 2, 3, 4, 5];
    let data_second = vec![1, 2, 3];
    save_data(&filename, &data_first, &data_second).unwrap();

    let (first, second) = load_data(filename).unwrap();
    println!("First row:  {:?}", first);
    println!("Second row: {:?}", second);
}
