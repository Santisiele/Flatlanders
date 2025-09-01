/// Archivo donde uno los tres mini crates 'teoricos'
use crate::flatlander::Flatlander;
use crate::geometria::longitud_de_sombra;
use crate::intervalo::Intervalo;

/// Funcion interna para ordenar los intervalos con el algoritmo de seleccion
///
/// No estaba seguro si se podia usar sortBy()
fn ordenar_por_inicio(intervalos: &mut [Intervalo]) {
    // me lo hace poner clippy, queria hacerlo como en la clase recibiendo un vector
    let n = intervalos.len();
    for i in 0..n {
        let mut min = i;
        for j in i + 1..n {
            if intervalos[j].inicio < intervalos[min].inicio {
                min = j;
            }
        }
        if min != i {
            intervalos.swap(i, min);
        }
    }
}

/// Devuelve la longitud de la union de los intervalos
///
/// Recibe un vector con los mismos
pub fn longitud_union(mut intervalos: Vec<Intervalo>) -> f64 {
    let n = intervalos.len();
    if n == 0 {
        return 0.0;
    }

    ordenar_por_inicio(&mut intervalos);

    let mut longitud_total = 0.0;
    let mut actual = Intervalo::nuevo(intervalos[0].inicio, intervalos[0].fin);

    for intervalo_iterado in intervalos.iter().take(n).skip(1) {
        if intervalo_iterado.inicio <= actual.fin {
            if intervalo_iterado.fin > actual.fin {
                actual.fin = intervalo_iterado.fin;
            }
        } else {
            longitud_total += actual.fin - actual.inicio;
            actual = Intervalo::nuevo(intervalo_iterado.inicio, intervalo_iterado.fin);
        }
    }
    longitud_total += actual.fin - actual.inicio;
    longitud_total
}

/// Devuelve un vector con las sombras de los flatlanders
///
/// Recibe el vector con los mismos y el angulo
pub fn sombras_desde_flatlanders(flatlanders: &[Flatlander], angulo_grados: f64) -> Vec<Intervalo> {
    let mut vector = Vec::with_capacity(flatlanders.len());
    for flatlander in flatlanders {
        let largo = longitud_de_sombra(flatlander.h, angulo_grados);
        vector.push(Intervalo::nuevo(flatlander.x, flatlander.x + largo));
    }
    vector
}

#[test]
fn ordena_por_inicio_funciona() {
    let mut vector = vec![
        Intervalo::nuevo(5.0, 6.0),
        Intervalo::nuevo(1.0, 2.0),
        Intervalo::nuevo(3.0, 4.0),
    ];
    ordenar_por_inicio(&mut vector);
    assert!(vector[0].inicio <= vector[1].inicio && vector[1].inicio <= vector[2].inicio);
}

#[test]
fn union_de_intervalos_solapados() {
    let vector = vec![Intervalo::nuevo(0.0, 10.0), Intervalo::nuevo(5.0, 15.0)];
    let total = longitud_union(vector);
    assert!((total - 15.0).abs() < 1e-12);
}

#[test]
fn union_de_intervalos_separados_y_juntos() {
    let vector = vec![
        Intervalo::nuevo(10.0, 12.0),
        Intervalo::nuevo(3.0, 5.0),
        Intervalo::nuevo(5.0, 8.0),
    ];
    let total = longitud_union(vector);
    assert!((total - 7.0).abs() < 1e-12);
}

#[test]
fn sombras_en_45_grados() {
    let flatlanders = vec![
        Flatlander { x: 0.0, h: 10.0 },
        Flatlander { x: 5.0, h: 10.0 },
    ];
    let sombras = sombras_desde_flatlanders(&flatlanders, 45.0);
    let total = longitud_union(sombras);
    assert!((total - 15.0).abs() < 1e-12);
}
