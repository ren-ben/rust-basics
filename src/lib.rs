pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn sum(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

pub fn flip(b: bool) -> bool {
    !b
}

// ownership & borrowing
pub fn take_ownership(s: String) -> usize {
    s.len()
}

pub fn borrow_first_char(s: &str) -> Option<char> {
    s.chars().next()
}

pub fn push_exclamation(s: &mut String) {
    s.push('!');
}

// ---------- 3. STRUCTS, ENUMS, METHODS ----------
// #[derive(...)] generiert automatisch Code für Traits:
// - Debug: Ermöglicht {:?} Formatierung
// - Clone: Erlaubt .clone() (Deep Copy)
// - Copy: Erlaubt implizites Kopieren (Bit-für-Bit, nur bei einfachen Typen möglich)
// - PartialEq: Ermöglicht == Vergleiche
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn origin() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

use std::fmt;
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Wir formatieren den Punkt als "(x, y)"
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
    Circle { center: Point, radius: f64 },
    Rect { top_left: Point, w: f64, h: f64 },
    Triangle { a: Point, b: Point, c: Point },
}

impl Shape {
    pub fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius, .. } => std::f64::consts::PI * radius * radius,
            Shape::Rect { w, h, .. } => w * h,
            Shape::Triangle { a, b, c } => {
                0.5 * ((a.x - c.x) * (b.y - a.y) - (a.x - b.x) * (c.y - a.y)).abs()
            }
        }
    }
}

// ---------- 4. TRAITS & GENERICS ----------
pub trait Plottable {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
}

impl Plottable for Point {
    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
}

impl Plottable for (f64, f64) {
    fn x(&self) -> f64 {
        self.0
    }
    fn y(&self) -> f64 {
        self.1
    }
}

// Return a reference to the item farthest from the origin.
pub fn furthest_from_origin<T: Plottable>(items: &[T]) -> Option<&T> {
    let mut best: Option<&T> = None;
    let mut best_d2 = -1.0_f64;

    for item in items {
        let d2 = item.x() * item.x() + item.y() * item.y();
        if d2 > best_d2 {
            best_d2 = d2;
            best = Some(item);
        }
    }

    best
}

pub fn min_by_key<T, K, F>(items: &[T], f: F) -> Option<&T>
where
    K: Ord,
    F: Fn(&T) -> K,
{
    if items.is_empty() {
        return None;
    }

    let mut min_item = &items[0];
    let mut min_val = f(min_item);

    for item in &items[1..] {
        let val = f(item);
        if val < min_val {
            min_val = val;
            min_item = item;
        }
    }
    Some(min_item)
}

// ---------- 5. ERRORS & OPTION/RESULT ----------
pub fn parse_port(s: &str) -> Result<u16, String> {
    s.parse::<u16>().map_err(|e| format!("Invalid port: {e}"))
}

// ---------- 6. ITERATORS & CLOSURES ----------
pub fn even_squares(n: u32) -> Vec<u32> {
    (0..=n).filter(|x| x % 2 == 0).map(|x| x * x).collect()
}

// ---------- 7. USING A CRATE (rand) ----------
pub fn roll_dice(sides: u8) -> u8 {
    use rand::Rng;
    rand::rng().random_range(1..=sides)
}
