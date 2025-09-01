use ejercicio_individual_1_santiagosielecki::{
    longitud_union::{longitud_union, sombras_desde_flatlanders},
    parseador::{leer_entrada, parsear_encabezado, parsear_flatlanders},
};

/// Centro de la ejecucion donde se hace la mezcla de todo
///
/// Imprime el resultado final
fn main() {
    let entrada = match leer_entrada() {
        Some(e) => e,
        None => return,
    };

    let mut lineas = entrada.lines();

    let (angulo, n) = match parsear_encabezado(lineas.next()) {
        Some(val) => val,
        None => return,
    };

    let flatlanders = match parsear_flatlanders(&mut lineas, n) {
        Some(v) => v,
        None => return,
    };

    let sombras = sombras_desde_flatlanders(&flatlanders, angulo);
    let total = longitud_union(sombras);
    println!("{:.13}", total);
}
