/// Representa un flatlander
///
/// Cada flatlander tiene su posicion x y su altura h
#[derive(Debug, PartialEq)]
pub struct Flatlander {
    /// Posicion x
    pub x: f64,
    /// Altura
    pub h: f64,
}

impl Flatlander {
    /// Crea el flatlander con los valores dados y lo devuelve
    pub fn nuevo(x: f64, h: f64) -> Self {
        Self { x, h }
    }
}

#[test]
fn crea_flatlander() {
    let flatlander = Flatlander::nuevo(10.0, 5.0);
    assert_eq!(flatlander.x, 10.0);
    assert_eq!(flatlander.h, 5.0);
}
