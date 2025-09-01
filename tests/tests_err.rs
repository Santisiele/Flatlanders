use ejercicio_individual_1_santiagosielecki::parseador::{
    leer_entrada_desde, parsear_encabezado, parsear_flatlanders,
};

/// Se testean casos con error
#[test]
fn err_encabezado_numero_invalido() {
    let entrada_str = "xx 2\n0 10\n0 10\n";
    let mut bytes = entrada_str.as_bytes();
    let entrada = leer_entrada_desde(&mut bytes).unwrap();

    let mut lineas = entrada.lines();
    let encabezado = parsear_encabezado(lineas.next());
    assert!(encabezado.is_none());
}

#[test]
fn err_angulo_fuera_de_rango_bajo() {
    let entrada_str = "5 2\n0 10\n0 10\n";
    let mut bytes = entrada_str.as_bytes();
    let entrada = leer_entrada_desde(&mut bytes).unwrap();

    let mut lineas = entrada.lines();
    let encabezado = parsear_encabezado(lineas.next());
    assert!(encabezado.is_none());
}

#[test]
fn err_angulo_fuera_de_rango_alto() {
    let entrada_str = "85 2\n0 10\n0 10\n";
    let mut bytes = entrada_str.as_bytes();
    let entrada = leer_entrada_desde(&mut bytes).unwrap();

    let mut lineas = entrada.lines();
    let encabezado = parsear_encabezado(lineas.next());
    assert!(encabezado.is_none());
}

#[test]
fn err_valor_faltante_en_encabezado() {
    let entrada_str = "45\n0 10\n0 10\n";
    let mut bytes = entrada_str.as_bytes();
    let entrada = leer_entrada_desde(&mut bytes).unwrap();

    let mut lineas = entrada.lines();
    let encabezado = parsear_encabezado(lineas.next());
    assert!(encabezado.is_none());
}

#[test]
fn err_linea_faltante_en_datos() {
    let entrada_str = "45 2\n0 10\n";
    let mut bytes = entrada_str.as_bytes();
    let entrada = leer_entrada_desde(&mut bytes).unwrap();

    let mut lineas = entrada.lines();
    let (_angulo, n) = match parsear_encabezado(lineas.next()) {
        Some(v) => v,
        None => return,
    };

    let salida = parsear_flatlanders(&mut lineas, n);
    assert!(salida.is_none());
}

#[test]
fn err_numero_invalido_en_x() {
    let entrada_str = "45 1\na 10\n";
    let mut bytes = entrada_str.as_bytes();
    let entrada = leer_entrada_desde(&mut bytes).unwrap();

    let mut lineas = entrada.lines();
    let (_angulo, n) = match parsear_encabezado(lineas.next()) {
        Some(v) => v,
        None => return,
    };

    let salida = parsear_flatlanders(&mut lineas, n);
    assert!(salida.is_none());
}

#[test]
fn err_numero_invalido_en_h() {
    let entrada_str = "45 1\n0 a\n";
    let mut bytes = entrada_str.as_bytes();
    let entrada = leer_entrada_desde(&mut bytes).unwrap();

    let mut lineas = entrada.lines();
    let (_angulo, n) = match parsear_encabezado(lineas.next()) {
        Some(v) => v,
        None => return,
    };

    let salida = parsear_flatlanders(&mut lineas, n);
    assert!(salida.is_none());
}
