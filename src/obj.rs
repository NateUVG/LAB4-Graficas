use crate::vector::Vec3;
use std::fs;


pub struct Obj {
    pub vertices: Vec<Vec3>,
    pub caras: Vec<Vec<usize>>,
}

impl Obj {
    pub fn cargar(ruta: &str) -> Result<Obj, String> {
        let texto = fs::read_to_string(ruta).map_err(|e| format!("no se pudo abrir {ruta}: {e}"))?;
        let mut vertices = Vec::new();
        let mut caras = Vec::new();
        for linea in texto.lines() {
            let mut partes = linea.split_whitespace();
            match partes.next() {
                Some("v") => {
                    let n: Vec<f32> = partes.take(3).map(|p| p.parse().unwrap_or(0.0)).collect();
                    vertices.push(Vec3::new(n[0], n[1], n[2]));
                }
                Some("f") => {
                    // Cada vertice de la cara viene como "v", "v/vt" o "v//vn":
                    let indices = partes
                        .filter_map(|p| p.split('/').next()?.parse::<usize>().ok())
                        .map(|i| i - 1)
                        .collect();
                    caras.push(indices);
                }
                _ => {}
            }
        }
        Ok(Obj { vertices, caras })
    }
}
