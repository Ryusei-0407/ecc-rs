#[derive(Debug, PartialEq)]
pub struct FieldElement {
    num: isize,
    prime: isize,
}

impl FieldElement {
    pub fn new(num: isize, prime: isize) -> FieldElement {
        if num >= prime || num < 0 {
            println!("Num {} not in field range 0 to {}", num, prime - 1)
        };

        FieldElement { num, prime }
    }

    pub fn add(&self, other: &FieldElement) -> FieldElement {
        FieldElement::new((self.num + other.num) % self.prime, self.prime)
    }

    pub fn sub(&self, other: &FieldElement) -> FieldElement {
        FieldElement::new((self.num - other.num + self.prime) % self.prime, self.prime)
    }
}

#[test]
fn ecc_new() {
    let a = FieldElement::new(7, 13);
    let b = FieldElement::new(6, 13);

    assert_eq!(a, a);
    assert_ne!(a, b);
}

#[test]
fn ecc_add() {
    let a = FieldElement::new(7, 13);
    let b = FieldElement::new(12, 13);
    let c = FieldElement::new(6, 13);

    assert_eq!(FieldElement::add(&a, &b), c);
}

#[test]
fn ecc_sub() {
    let a = FieldElement::new(7, 13);
    let b = FieldElement::new(12, 13);
    let c = FieldElement::new(5, 13);

    assert_eq!(FieldElement::sub(&b, &a), c);
}