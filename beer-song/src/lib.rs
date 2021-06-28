pub fn verse(n: u32) -> String {
    match n {
        0 => {
            let v = "No more bottles of beer on the wall, no more bottles of beer.\n\
                     Go to the store and buy some more, 99 bottles of beer on the wall.\n";
            v.to_string()
        }
        1 => {
            let v = "1 bottle of beer on the wall, 1 bottle of beer.\n\
                     Take it down and pass it around, no more bottles of beer on the wall.\n";
            v.to_string()
        }
        2 => {
            let v = "2 bottles of beer on the wall, 2 bottles of beer.\n\
                     Take one down and pass it around, 1 bottle of beer on the wall.\n";
            v.to_string()
        }
        _ => {
            let v = format!(
                "{} bottles of beer on the wall, {0} bottles of beer.\n\
                Take one down and pass it around, {} bottles of beer on the wall.\n",
                n,
                n - 1
            );
            v.to_string()
        }
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song = String::new();
    for n in (end..start + 1).rev() {
        song.push_str(format!("{}\n", verse(n)).as_str());
    }
    song.pop();
    song
}
