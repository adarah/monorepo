use std::error::Error;

const MAX_IMG_WIDTH: usize = 256;
const MAX_IMG_HEIGHT: usize = 256;

fn main() -> Result<(), Box<dyn Error>> {
    print!("P3\n{MAX_IMG_WIDTH} {MAX_IMG_HEIGHT}\n255\n");
    for i in 0..MAX_IMG_HEIGHT {
        let remaining = MAX_IMG_HEIGHT - i;
        eprintln!("Lines remaining: {remaining}");
        for j in 0..MAX_IMG_WIDTH {
            println!("{i} {j} 100");
        }
    }

    Ok(())
}
