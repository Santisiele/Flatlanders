use ejercicio_individual_1_santiagosielecki::{
    longitud_union::{longitud_union, sombras_desde_flatlanders},
    parseador::{leer_entrada_desde, parsear_encabezado, parsear_flatlanders},
};

/// Funcion para confirmar que este dentro del error permitido que es 10 a la -4
fn aproximadamente_iguales(a: f64, b: f64, tolerancia: f64) -> bool {
    let diferencia = (a - b).abs();
    diferencia <= tolerancia.max(1e-12) || diferencia / b.abs().max(1.0) <= tolerancia
}

/// Testeo usando traits (se dijo en DS)
/// Se testean casos donde la ejecucion sale bien
#[test]
fn ok_superposicion_simple_45() {
    let entrada_str = "45 2\n0 10\n5 10\n";
    let mut bytes = entrada_str.as_bytes();
    let entrada = match leer_entrada_desde(&mut bytes) {
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

    assert!(aproximadamente_iguales(total, 15.0, 1e-4));
}

#[test]
fn ok_ejemplo_30_grados() {
    let entrada_str = "30 3\n50 150\n0 100\n100 200\n";
    let mut bytes = entrada_str.as_bytes();
    let entrada = leer_entrada_desde(&mut bytes).unwrap();

    let mut lineas = entrada.lines();
    let (angulo, n) = parsear_encabezado(lineas.next()).unwrap();
    let flatlanders = parsear_flatlanders(&mut lineas, n).unwrap();

    let sombras = sombras_desde_flatlanders(&flatlanders, angulo);
    let total = longitud_union(sombras);

    let esperado = 446.4101615137755_f64;
    assert!(aproximadamente_iguales(total, esperado, 1e-4));
}

#[test]
fn ok_ejemplo_45_grados() {
    let entrada_str = "45 3\n50 150\n0 100\n100 200\n";
    let mut bytes = entrada_str.as_bytes();
    let entrada = leer_entrada_desde(&mut bytes).unwrap();

    let mut lineas = entrada.lines();
    let (angulo, n) = parsear_encabezado(lineas.next()).unwrap();
    let flatlanders = parsear_flatlanders(&mut lineas, n).unwrap();

    let sombras = sombras_desde_flatlanders(&flatlanders, angulo);
    let total = longitud_union(sombras);

    let esperado = 300.00000000000006_f64;
    assert!(aproximadamente_iguales(total, esperado, 1e-4));
}

#[test]
fn ok_sin_solapamiento_45() {
    let entrada_str = "45 2\n0 10\n20 5\n";
    let mut bytes = entrada_str.as_bytes();
    let entrada = leer_entrada_desde(&mut bytes).unwrap();

    let mut lineas = entrada.lines();
    let (angulo, n) = parsear_encabezado(lineas.next()).unwrap();
    let flatlanders = parsear_flatlanders(&mut lineas, n).unwrap();

    let sombras = sombras_desde_flatlanders(&flatlanders, angulo);
    let total = longitud_union(sombras);

    assert!(aproximadamente_iguales(total, 15.0, 1e-4));
}
