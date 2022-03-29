mod vec3;

use anyhow::Result;
use vec3::Color;

const MAX_IMG_WIDTH: usize = 256;
const MAX_IMG_HEIGHT: usize = 256;

fn main() -> Result<()> {
    print!("P3\n{MAX_IMG_WIDTH} {MAX_IMG_HEIGHT}\n255\n");
    for i in 0..MAX_IMG_HEIGHT {
        let remaining = MAX_IMG_HEIGHT - i;
        eprintln!("Lines remaining: {remaining}");
        for j in 0..MAX_IMG_WIDTH {
            println!("{}", Color::new((i as f64)/ MAX_IMG_HEIGHT as f64, (j as f64) / MAX_IMG_WIDTH as f64, 0.5));
        }
    }


    Ok(())
}


#[cfg(test)]
mod tests {
    #[test]
    fn make_bazel_happy() {
        assert_eq!(1 + 1, 2);
    }
}
