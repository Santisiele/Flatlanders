use std::f64::consts::PI;

/// Convierte los grados a radianes
///
/// Solo se usa de manera interna
fn convertir_grados_a_radianes(grados: f64) -> f64 {
    grados * PI / 180.0
}

/// Calcula la tangente de un angulo, que se recibe en formato grados
pub fn tangente(angulo: f64) -> f64 {
    convertir_grados_a_radianes(angulo).tan()
}

/// Calcula la longitud de una sombra dada la altura y el angulo, que tambien se recibe en grados porque se pasa a radianes de manera interna
pub fn longitud_de_sombra(h: f64, angulo: f64) -> f64 {
    h / tangente(angulo)
}

#[test]
fn tangente_45_grados_es_uno() {
    let tangente = tangente(45.0);
    assert!((tangente - 1.0).abs() < 1e-12); // en la salida final es 10 a la -4 pero aca soy mas estricto
}

#[test]
fn sombra_en_45_grados_igual_a_altura() {
    let h = 10.0;
    let longitud = longitud_de_sombra(h, 45.0);
    assert!((longitud - h).abs() < 1e-12);
}

#[test]
fn tangente_crece_entre_10_y_80() {
    let tangente10 = tangente(10.0);
    let tangente80 = tangente(80.0);
    assert!(tangente10 < tangente80);
    assert!((tangente10 - 0.176_326_980_708_465).abs() < 1e-12);
    assert!((tangente80 - 5.671_281_819_617_709).abs() < 1e-12);
}

#[test]
fn sombra_disminuye_con_el_angulo_en_el_rango_valido() {
    let h = 12.3;
    let longitud10 = longitud_de_sombra(h, 10.0);
    let longitud80 = longitud_de_sombra(h, 80.0);
    assert!(longitud10 > longitud80);
}

#[test]
fn comparacion_con_formula_directa_en_radianes() {
    let angulo = 33.7;
    let tangente_directa = (angulo * PI / 180.0).tan();
    let tangente_funcion = tangente(angulo);
    assert!((tangente_directa - tangente_funcion).abs() < 1e-12);
}
