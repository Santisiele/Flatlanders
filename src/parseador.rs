/// Archivo para parsear las entradas del stdin
use crate::flatlander::Flatlander;
use std::{
    io::{self},
    str::FromStr,
};

/// Constantes definidas para los errores
const ERR_IO: &str = "\"IO\"";
const ERR_LINEA_FALTANTE: &str = "\"Linea faltante\"";
const ERR_VALOR_FALTANTE: &str = "\"Valor faltante\"";
const ERR_NUMERO_INVALIDO: &str = "\"Numero invalido\"";
const ERR_FUERA_DE_RANGO: &str = "\"Fuera de rango\"";

/// Parametros definidos por la consigna
const N_MIN: usize = 1;
const N_MAX: usize = 100_000;
const X_MIN: f64 = 0.0;
const X_MAX: f64 = 300_000.0;
const H_MIN: f64 = 1.0;
const H_MAX: f64 = 1000.0;

/// Funcion para poder usar en los tests
///
/// Se le dice de donde se lee la entrada
pub fn leer_entrada_desde(mut entrada: impl std::io::Read) -> Option<String> {
    let mut s = String::new();
    if entrada.read_to_string(&mut s).is_err() {
        eprintln!("Error: {ERR_IO}");
        return None;
    }
    Some(s)
}

/// Lee toda la entrada como un string
///
/// Si hay un error devuelve None
///
/// Usa la funcion de arriba y se dice que se lea desde el stdin
pub fn leer_entrada() -> Option<String> {
    leer_entrada_desde(io::stdin())
}

/// Funcion privada para validar las entradas y no pasarme de 30 lineas en los parseos y borrar codigo repetido
fn validar_str<'a>(lineas_split: &mut std::str::SplitWhitespace<'a>) -> Option<&'a str> {
    match lineas_split.next() {
        Some(s) => Some(s),
        None => {
            eprintln!("Error: {ERR_VALOR_FALTANTE}");
            None
        }
    }
}

/// Funcion privada para parsear los str y no pasarme de 30 lineas en los y borrar codigo repetido
///
/// A veces necesito f64 y a veces usize, por eso uso tipo generico T
fn parsear_str_a_numero<T: FromStr>(numero_str: &str) -> Option<T> {
    match numero_str.parse::<T>() {
        Ok(v) => Some(v),
        Err(_) => {
            eprintln!("Error: {ERR_NUMERO_INVALIDO}");
            None
        }
    }
}

/// Funcion privada para crear el encabezado
///
/// Se valida tambien que el angulo este entre 10 y 80
///
/// Imprime el error en caso de ser necesario
fn crear_encabezado(lineas_split: &mut std::str::SplitWhitespace<'_>) -> Option<(f64, usize)> {
    let ang_str = validar_str(lineas_split)?;
    let n_str = validar_str(lineas_split)?;
    let angulo: f64 = parsear_str_a_numero(ang_str)?;
    if !(10.0..=80.0).contains(&angulo) {
        eprintln!("Error: {ERR_FUERA_DE_RANGO}");
        return None;
    }
    let n: usize = parsear_str_a_numero(n_str)?;
    if !(N_MIN..=N_MAX).contains(&n) {
        eprintln!("Error: {ERR_FUERA_DE_RANGO}");
        return None;
    }
    Some((angulo, n))
}

/// Parsea el encabezado usando la funcion privada
pub fn parsear_encabezado(linea_opt: Option<&str>) -> Option<(f64, usize)> {
    let linea = match linea_opt {
        Some(l) => l,
        None => {
            eprintln!("Error: {ERR_LINEA_FALTANTE}");
            return None;
        }
    };
    let mut lineas_split = linea.split_whitespace();

    let encabezado = crear_encabezado(&mut lineas_split)?; // me lo hace poner clippy, habia usado un match

    Some(encabezado)
}

/// Funcion privada para crear el flatlander
///
/// Imprime el error en caso de ser necesario
fn crear_flatlander(lineas_split: &mut std::str::SplitWhitespace<'_>) -> Option<Flatlander> {
    let x_str = validar_str(lineas_split)?;
    let h_str = validar_str(lineas_split)?;
    let x: f64 = parsear_str_a_numero(x_str)?;
    let h: f64 = parsear_str_a_numero(h_str)?;
    if !(X_MIN..=X_MAX).contains(&x) || !(H_MIN..=H_MAX).contains(&h) {
        eprintln!("Error: {ERR_FUERA_DE_RANGO}");
        return None;
    }
    Some(Flatlander { x, h })
}

/// Parsea los flatlanders usando la funcion privada
pub fn parsear_flatlanders(lineas: &mut std::str::Lines<'_>, n: usize) -> Option<Vec<Flatlander>> {
    let mut flatlanders = Vec::with_capacity(n);

    for _ in 0..n {
        let linea = match lineas.next() {
            Some(l) => l,
            None => {
                eprintln!("Error: {ERR_LINEA_FALTANTE}");
                return None;
            }
        };

        let mut lineas_split = linea.split_whitespace();

        let flatlander = crear_flatlander(&mut lineas_split)?; // lo mismo que en la linea 97
        flatlanders.push(flatlander);
    }
    Some(flatlanders)
}

#[test]
fn parsear_encabezado_ok() {
    let linea = "45 3";
    let salida = parsear_encabezado(Some(linea));
    assert_eq!(salida, Some((45.0, 3)));
}

#[test]
fn parsear_encabezado_linea_faltante() {
    let salida = parsear_encabezado(None);
    assert!(salida.is_none());
}

#[test]
fn parsear_encabezado_valor_faltante() {
    let linea = "45";
    let salida = parsear_encabezado(Some(linea));
    assert!(salida.is_none());
}

#[test]
fn parsear_encabezado_numero_invalido() {
    let linea = "xx 2";
    let salida = parsear_encabezado(Some(linea));
    assert!(salida.is_none());
}

#[test]
fn parsear_encabezado_fuera_de_rango_bajo() {
    let linea = "5 2";
    let salida = parsear_encabezado(Some(linea));
    assert!(salida.is_none());
}

#[test]
fn parsear_encabezado_fuera_de_rango_alto() {
    let linea = "85 2";
    let salida = parsear_encabezado(Some(linea));
    assert!(salida.is_none());
}

#[test]
fn parsear_flatlanders_ok() {
    let entrada_str = "0 10\n5 20\n";
    let mut lineas = entrada_str.lines();
    let salida = parsear_flatlanders(&mut lineas, 2).unwrap();
    assert_eq!(salida.len(), 2);
    assert!((salida[0].x - 0.0).abs() < 1e-12 && (salida[0].h - 10.0).abs() < 1e-12);
    assert!((salida[1].x - 5.0).abs() < 1e-12 && (salida[1].h - 20.0).abs() < 1e-12);
}

#[test]
fn parsear_flatlanders_linea_faltante() {
    let entrada_str = "0 10\n";
    let mut lineas = entrada_str.lines();
    let salida = parsear_flatlanders(&mut lineas, 2);
    assert!(salida.is_none());
}

#[test]
fn parsear_flatlanders_valor_faltante() {
    let entrada_str = "0\n";
    let mut lineas = entrada_str.lines();
    let salida = parsear_flatlanders(&mut lineas, 1);
    assert!(salida.is_none());
}

#[test]
fn parsear_flatlanders_numero_invalido_en_x() {
    let entrada_str = "a 10\n";
    let mut lineas = entrada_str.lines();
    let salida = parsear_flatlanders(&mut lineas, 1);
    assert!(salida.is_none());
}

#[test]
fn parsear_flatlanders_numero_invalido_en_h() {
    let entrada_str = "0 a\n";
    let mut lineas = entrada_str.lines();
    let salida = parsear_flatlanders(&mut lineas, 1);
    assert!(salida.is_none());
}

#[test]
fn parsear_encabezado_n_fuera_de_rango_bajo() {
    let linea = "45 0";
    let salida = parsear_encabezado(Some(linea));
    assert!(salida.is_none());
}

#[test]
fn parsear_encabezado_n_fuera_de_rango_alto() {
    let linea = "45 100001";
    let salida = parsear_encabezado(Some(linea));
    assert!(salida.is_none());
}

#[test]
fn parsear_flatlanders_x_fuera_de_rango_alto() {
    let entrada_str = "300001 10\n";
    let mut lineas = entrada_str.lines();
    let salida = parsear_flatlanders(&mut lineas, 1);
    assert!(salida.is_none());
}

#[test]
fn parsear_flatlanders_h_fuera_de_rango_bajo() {
    let entrada_str = "0 0\n";
    let mut lineas = entrada_str.lines();
    let salida = parsear_flatlanders(&mut lineas, 1);
    assert!(salida.is_none());
}

#[test]
fn parsear_flatlanders_h_fuera_de_rango_alto() {
    let entrada_str = "0 1001\n";
    let mut lineas = entrada_str.lines();
    let salida = parsear_flatlanders(&mut lineas, 1);
    assert!(salida.is_none());
}

#[test]
fn parsear_flatlanders_espacios_y_linea_en_blanco() {
    let entrada_str = "   0    10   \n5    20   \n\n";
    let mut lineas = entrada_str.lines();
    let salida = parsear_flatlanders(&mut lineas, 2).unwrap();
    assert_eq!(salida.len(), 2);
    assert!((salida[0].x - 0.0).abs() < 1e-12 && (salida[0].h - 10.0).abs() < 1e-12);
    assert!((salida[1].x - 5.0).abs() < 1e-12 && (salida[1].h - 20.0).abs() < 1e-12);
}
