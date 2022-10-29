struct RGB(i32, i32, i32);
struct CMYK(i32, i32, i32);

fn main() {
    let color1 = RGB(256, 0, 0);
    let color2 = CMYK(3000, 10, 10, 10);
}
