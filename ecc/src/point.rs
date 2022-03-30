mod fieldelement;

use fieldelement::*;

#[derive(Debug, PartialEq)]
pub struct Point {
    x: FieldElement,
    y: FieldElement,
    a: FieldElement,
    b: FieldElement,
}

impl Point {
    fn new(x: FieldElement, y: FieldElement, a: FieldElement, b: FieldElement) -> Point {
        if x.num == 0 && y.num == 0 {
            return Point { x, y, a, b };
        } else if a.num == 0
            && FieldElement::pow(&y, 2) != FieldElement::add(&FieldElement::pow(&x, 3), &b)
        {
            panic!("({:?}, {:?}) is not on the curve", x, y,)
        } else if FieldElement::pow(&y, 2)
            != FieldElement::add(
                &FieldElement::add(&FieldElement::pow(&x, 3), &FieldElement::mul(&a, &x)),
                &b,
            )
        {
            panic!("({:?}, {:?}) is not on the curve", x, y)
        } else {
            return Point { x, y, a, b };
        }
    }

    fn add(&self, other: &Point) -> Point {
        if self.a.num != other.a.num || self.b.num != other.b.num {
            panic!("Points {:?}, {:?} are not on the same curve", self, other)
        };
        let a = &self.a;
        let b = &self.b;
        let prime = &a.prime;

        if self == other && self.y.num == 0 {
            return Point::new(
                FieldElement::new(0, *prime),
                FieldElement::new(0, *prime),
                *a,
                *b,
            );
        };

        if self.x == other.x && self.y != other.y {
            return Point::new(
                FieldElement::new(0, *prime),
                FieldElement::new(0, *prime),
                *a,
                *b,
            );
        };

        let x = (self.x.num, other.x.num);
        match x {
            (0, ..) => {
                let (x, y) = (&other.x, &other.y);
                Point::new(*x, *y, *a, *b)
            }
            (.., 0) => Point::new(self.x, self.y, *a, *b),
            _ => {
                let s = if self.x != other.x {
                    FieldElement::mul(
                        &FieldElement::sub(&other.y, &self.y),
                        &FieldElement::div(&FieldElement::sub(&other.x, &self.x), -1),
                    )
                } else if self.x == other.x && self.y == other.y {
                    FieldElement::mul(
                        &FieldElement::add(
                            &FieldElement::mul(
                                &FieldElement::pow(&self.x, 2),
                                &FieldElement::new(3, *prime),
                            ),
                            a,
                        ),
                        &FieldElement::div(
                            &FieldElement::mul(&self.y, &FieldElement::new(2, *prime)),
                            -1,
                        ),
                    )
                } else {
                    FieldElement::mul(
                        &FieldElement::sub(&other.y, &self.y),
                        &FieldElement::div(&FieldElement::sub(&other.x, &self.x), -1),
                    )
                };
                let x = FieldElement::sub(
                    &FieldElement::sub(&FieldElement::pow(&s, 2), &self.x),
                    &other.x,
                );
                let y = FieldElement::sub(
                    &FieldElement::mul(&s, &FieldElement::sub(&self.x, &x)),
                    &self.y,
                );

                Point::new(x, y, *a, *b)
            }
        }
    }
}

#[test]
fn point_new1() {
    let prime = 223;
    let a = FieldElement::new(0, prime);
    let b = FieldElement::new(7, prime);
    let (x, y) = (FieldElement::new(192, prime), FieldElement::new(105, prime));

    let _p1 = Point::new(x, y, a, b);
}

#[test]
fn point_new2() {
    let prime = 223;
    let a = FieldElement::new(0, prime);
    let b = FieldElement::new(7, prime);
    let (x, y) = (FieldElement::new(17, prime), FieldElement::new(56, prime));

    let _p2 = Point::new(x, y, a, b);
}

#[test]
#[should_panic]
fn point_new3() {
    let prime = 223;
    let a = FieldElement::new(0, prime);
    let b = FieldElement::new(7, prime);
    let (x, y) = (FieldElement::new(200, prime), FieldElement::new(119, prime));

    let _p3 = Point::new(x, y, a, b);
}

#[test]
fn point_new4() {
    let prime = 223;
    let a = FieldElement::new(0, prime);
    let b = FieldElement::new(7, prime);
    let (x, y) = (FieldElement::new(1, prime), FieldElement::new(193, prime));

    let _p4 = Point::new(x, y, a, b);
}

#[test]
#[should_panic]
fn point_new5() {
    let prime = 223;
    let a = FieldElement::new(0, prime);
    let b = FieldElement::new(7, prime);
    let (x, y) = (FieldElement::new(42, prime), FieldElement::new(99, prime));

    let _p5 = Point::new(x, y, a, b);
}

#[test]
fn point_add() {
    let prime = 223;
    let a = FieldElement::new(0, prime);
    let b = FieldElement::new(7, prime);
    let (x1, y1) = (FieldElement::new(192, prime), FieldElement::new(105, prime));
    let (x2, y2) = (FieldElement::new(17, prime), FieldElement::new(56, prime));

    let p1 = Point::new(x1, y1, a, b);
    let p2 = Point::new(x2, y2, a, b);

    let x = FieldElement::new(170, prime);
    let y = FieldElement::new(142, prime);
    let ans = Point::new(x, y, a, b);

    assert_eq!(Point::add(&p1, &p2), ans);
}
