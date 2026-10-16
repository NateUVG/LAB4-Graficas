mod framebuffer;
mod obj;
mod triangle;
mod vector;

use framebuffer::{Color, FrameBuffer};
use minifb::{Key, Window, WindowOptions};
use obj::Obj;
use triangle::triangle;
use vector::Vec3;

const SCREEN_WIDTH: usize = 900;
const SCREEN_HEIGHT: usize = 700;
const VELOCIDAD_GIRO: f32 = 0.03;


fn a_pantalla(v: Vec3, centro: Vec3, escala: f32, giro_y: f32, giro_x: f32) -> Vec3 {
    let p = (v - centro).rotar(giro_y, giro_x) * escala;
    Vec3::new(SCREEN_WIDTH as f32 / 2.0 + p.x, SCREEN_HEIGHT as f32 / 2.0 - p.y, p.z)
}

fn render(fb: &mut FrameBuffer, modelo: &Obj, centro: Vec3, escala: f32, giro_y: f32, giro_x: f32) {
    // Se recorren las caras: con los indices de cada una se buscan sus tres
    // vertices y se dibuja el triangulo.
    for cara in &modelo.caras {
        for k in 1..cara.len() - 1 {
            let a = modelo.vertices[cara[0]];
            let b = modelo.vertices[cara[k]];
            let c = modelo.vertices[cara[k + 1]];
            triangle(
                fb,
                a_pantalla(a, centro, escala, giro_y, giro_x),
                a_pantalla(b, centro, escala, giro_y, giro_x),
                a_pantalla(c, centro, escala, giro_y, giro_x),
            );
        }
    }
}

fn main() {
    let modelo = Obj::cargar("assets/nave.obj").unwrap_or_else(|e| panic!("{e}"));

    // Centro y tamaño del modelo, para que quede en el medio de la ventana.
    let (mut minimo, mut maximo) = (modelo.vertices[0], modelo.vertices[0]);
    for &v in &modelo.vertices {
        minimo = minimo.min(v);
        maximo = maximo.max(v);
    }
    let centro = (minimo + maximo) * 0.5;
    let tamano = maximo - minimo;
    let escala = 0.6 * (SCREEN_HEIGHT as f32) / tamano.x.max(tamano.y).max(tamano.z);

    let mut framebuffer = FrameBuffer::new(SCREEN_WIDTH, SCREEN_HEIGHT, Color::new(10, 10, 30));
    framebuffer.set_background_color(Color::new(10, 10, 30));

    let mut window = Window::new("Lab 4 - Modelo OBJ", SCREEN_WIDTH, SCREEN_HEIGHT, WindowOptions::default())
        .expect("no se pudo crear la ventana");
    window.set_target_fps(60);

    let (mut giro_y, mut giro_x) = (0.7, -0.45);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::Left) {
            giro_y -= VELOCIDAD_GIRO;
        }
        if window.is_key_down(Key::Right) {
            giro_y += VELOCIDAD_GIRO;
        }
        if window.is_key_down(Key::Up) {
            giro_x += VELOCIDAD_GIRO;
        }
        if window.is_key_down(Key::Down) {
            giro_x -= VELOCIDAD_GIRO;
        }

        framebuffer.clear();
        framebuffer.set_current_color(Color::new(255, 255, 0));
        render(&mut framebuffer, &modelo, centro, escala, giro_y, giro_x);

        let (w, h) = framebuffer.dimensions();
        window.update_with_buffer(framebuffer.buffer(), w, h).expect("no se pudo actualizar la ventana");
    }
}
