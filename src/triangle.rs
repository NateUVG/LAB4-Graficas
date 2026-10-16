use crate::framebuffer::FrameBuffer;
use crate::vector::Vec3;

/// Linea de Bresenham entre dos puntos de la pantalla.
pub fn line(fb: &mut FrameBuffer, a: Vec3, b: Vec3) {
    let (mut x0, mut y0) = (a.x.round() as i32, a.y.round() as i32);
    let (x1, y1) = (b.x.round() as i32, b.y.round() as i32);
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut error = dx + dy;
    loop {
        fb.point(x0, y0);
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * error;
        if e2 >= dy {
            error += dy;
            x0 += sx;
        }
        if e2 <= dx {
            error += dx;
            y0 += sy;
        }
    }
}

/// Triangulo: sus tres lados.
pub fn triangle(fb: &mut FrameBuffer, a: Vec3, b: Vec3, c: Vec3) {
    line(fb, a, b);
    line(fb, b, c);
    line(fb, c, a);
}
