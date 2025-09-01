/// Representa un intervalo en la recta
///
/// Cada intervalo tiene su extremo inicio (izquierdo) y fin (derecho)
#[derive(Debug, PartialEq)]
pub struct Intervalo {
    /// Extremo izquierdo
    pub inicio: f64,
    ///Extremo derecho
    pub fin: f64,
}

impl Intervalo {
    /// Crea un intervalo con los valores dados y lo devuelve
    ///
    /// Como en la consigna no lo especificaba, los intervalos son siempre de izquierda a derecha por lo que si fin es menor que inicio, se da vuelta
    pub fn nuevo(inicio: f64, fin: f64) -> Self {
        if inicio <= fin {
            Self { inicio, fin }
        } else {
            Self {
                inicio: fin,
                fin: inicio,
            }
        }
    }

    /// Devuelve la longitud del intervalo fin-inicio
    pub fn longitud(&self) -> f64 {
        self.fin - self.inicio
    }

    /// Devuelve true si dos intervalos se solapan y false sino
    ///
    /// Se solapan si hay por lo menos un punto en comun
    pub fn se_solapa(&self, segundo_intervalo: &Intervalo) -> bool {
        self.inicio <= segundo_intervalo.fin && segundo_intervalo.inicio <= self.fin
    }

    /// Une el intervalo seleccionado con otro
    ///
    /// Si estan solapados o son adyacentes, el nuevo intervalo es la union
    /// Si estan separados, es el intervalo minimo que los tiene a los dos
    pub fn unir(&mut self, segundo_intervalo: &Intervalo) {
        if segundo_intervalo.inicio < self.inicio {
            self.inicio = segundo_intervalo.inicio;
        }
        if segundo_intervalo.fin > self.fin {
            self.fin = segundo_intervalo.fin;
        }
    }
}

#[test]
fn crea_intervalo_y_mide_longitud() {
    let intervalo = Intervalo::nuevo(1.0, 5.0);
    assert_eq!(intervalo.inicio, 1.0);
    assert_eq!(intervalo.fin, 5.0);
    assert_eq!(intervalo.longitud(), 4.0);
}

#[test]
fn crea_intervalo_y_mide_longitud_al_reves() {
    let intervalo = Intervalo::nuevo(5.0, 1.0);
    assert_eq!(intervalo.longitud(), 4.0);
}

#[test]
fn intervalos_dentro_de_otro_devuelve_solapado_verdadero() {
    let intervalo_1 = Intervalo::nuevo(1.0, 5.0);
    let intervalo_2 = Intervalo::nuevo(2.0, 4.0);
    assert!(intervalo_1.se_solapa(&intervalo_2))
}

#[test]
fn intervalos_parcialmente_solapados_devuelve_solapado_verdadero() {
    let intervalo_1 = Intervalo::nuevo(1.0, 5.0);
    let intervalo_2 = Intervalo::nuevo(3.0, 6.0);
    assert!(intervalo_1.se_solapa(&intervalo_2))
}

#[test]
fn intervalos_no_solapados_devuelve_solapado_falso() {
    let intervalo_1 = Intervalo::nuevo(1.0, 5.0);
    let intervalo_2 = Intervalo::nuevo(6.0, 8.0);
    assert!(!intervalo_1.se_solapa(&intervalo_2))
}

#[test]
fn intervalos_juntos_devuelve_solapado_verdadero() {
    let intervalo_1 = Intervalo::nuevo(1.0, 5.0);
    let intervalo_2 = Intervalo::nuevo(5.0, 8.0);
    assert!(intervalo_1.se_solapa(&intervalo_2))
}

#[test]
fn une_solapados_correctamente() {
    let mut intervalo_1 = Intervalo::nuevo(1.0, 5.0);
    let intervalo_2 = Intervalo::nuevo(5.0, 8.0);
    intervalo_1.unir(&intervalo_2);
    assert_eq!(intervalo_1.inicio, 1.0);
    assert_eq!(intervalo_1.fin, 8.0);
}

#[test]
fn intervalos_dentro_de_otro_se_unen_correctamente() {
    let mut intervalo_1 = Intervalo::nuevo(1.0, 5.0);
    let intervalo_2 = Intervalo::nuevo(2.0, 4.0);
    intervalo_1.unir(&intervalo_2);
    assert_eq!(intervalo_1.inicio, 1.0);
    assert_eq!(intervalo_1.fin, 5.0);
}

#[test]
fn intervalos_separados_se_unen_correctamente() {
    let mut intervalo_1 = Intervalo::nuevo(4.0, 5.0);
    let intervalo_2 = Intervalo::nuevo(2.0, 3.0);
    intervalo_1.unir(&intervalo_2);
    assert_eq!(intervalo_1.inicio, 2.0);
    assert_eq!(intervalo_1.fin, 5.0);
}
