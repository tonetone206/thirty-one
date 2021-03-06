extern crate rand;

use std::fmt;

use rand::StdRng;
use std::cmp::Ordering;
use std::cmp::Ordering::{Less, Equal, Greater};

pub struct Platform {
    pub print_xy: fn(i32, i32, &str),
    pub clear: fn(Option<Rect>),
    pub size: fn() -> Size,
    pub pick: fn(Point, i32) -> char,
    pub mouse_position: fn() -> Point,
    pub clicks: fn() -> i32,
    pub key_pressed: fn(KeyCode) -> bool,
    pub set_colors: fn(Color, Color),
    pub get_colors: fn() -> (Color, Color),
    pub set_foreground: fn(Color),
    pub get_foreground: fn() -> (Color),
    pub set_background: fn(Color),
    pub get_background: fn() -> (Color),
    pub set_layer: fn(i32),
    pub get_layer: fn() -> i32,
}

pub struct State {
    pub rng: StdRng,
    pub title_screen: bool,
    pub deck: Vec<Card>,
    pub pile: Vec<Card>,
    pub player: HandEnum,
    pub cpu_players: Vec<HandEnum>,
    pub turn: Turn,
    pub turn_count: u32,
    pub summary: String,
    pub ui_context: UIContext,
}

#[derive(Clone)]
pub enum Turn {
    PlayerTurn(Option<Participant>), //possible knocker
    PlayerSelected(Card, Option<Participant>), //possible knocker
    CpuTurn(Option<Participant>), //possible knocker
    CpuSummary(Option<KnockerOrWinner>),
    Resolution(Option<Participant>), //possible Winner
}

#[derive(Clone, PartialEq, Debug)]
pub enum Participant {
    Player,
    Cpu(usize),
}
use Participant::*;

impl fmt::Display for Participant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "{}", match *self {
            Player => "You".to_string(),
            Cpu(i) => format!("Cpu {}", i),
        })
    }
}

pub type UiId = i32;

pub struct UIContext {
    pub hot: UiId,
    pub active: UiId,
    pub next_hot: UiId,
}

impl UIContext {
    pub fn new() -> Self {
        UIContext {
            hot: 0,
            active: 0,
            next_hot: 0,
        }
    }

    pub fn set_not_active(&mut self) {
        self.active = 0;
    }
    pub fn set_active(&mut self, id: UiId) {
        self.active = id;
    }
    pub fn set_next_hot(&mut self, id: UiId) {
        self.next_hot = id;
    }
    pub fn set_not_hot(&mut self) {
        self.hot = 0;
    }
    pub fn frame_init(&mut self) {
        if self.active == 0 {
            self.hot = self.next_hot;
        }
        self.next_hot = 0;
    }
}

pub trait AllValues {
    fn all_values() -> Vec<Self> where Self: std::marker::Sized;
}

pub enum HandEnum {
    Hand(Card, Card, Card),
}
use HandEnum::*;

impl fmt::Display for HandEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "{}", match *self {
            Hand(ref c1, ref c2, ref c3) => format!("a {}, a {} and a {}", c1, c2, c3),
        })
    }
}

impl HandEnum {
    pub fn swap(&mut self, index: HandCard, new_card: Card) -> Card {
        match self {
            &mut Hand(ref mut c1, ref mut c2, ref mut c3) => {
                match index {
                    FirstCard => {
                        let result = c1.clone();
                        *c1 = new_card;
                        result
                    }
                    SecondCard => {
                        let result = c2.clone();
                        *c2 = new_card;
                        result
                    }
                    ThirdCard => {
                        let result = c3.clone();
                        *c3 = new_card;
                        result
                    }
                }
            }
        }
    }

    pub fn score(&self) -> Score {
        match self {
            &Hand(ref c1, ref c2, ref c3) => score_cards(c1, c2, c3),
        }
    }
    pub fn is_31(&self) -> bool {
        match self.score() {
            Simple(x) => x >= 31,
            _ => false,
        }
    }
}

pub fn score_cards(c1: &Card, c2: &Card, c3: &Card) -> Score {
    if c1.value == c2.value && c2.value == c3.value {
        ThirtyAndAHalf
    } else {
        let mut clubs = Vec::new();
        let mut diamonds = Vec::new();
        let mut hearts = Vec::new();
        let mut spades = Vec::new();

        match c1.suit {
            Clubs => clubs.push(c1.value),
            Diamonds => diamonds.push(c1.value),
            Hearts => hearts.push(c1.value),
            Spades => spades.push(c1.value),
        }
        match c2.suit {
            Clubs => clubs.push(c2.value),
            Diamonds => diamonds.push(c2.value),
            Hearts => hearts.push(c2.value),
            Spades => spades.push(c2.value),
        }
        match c3.suit {
            Clubs => clubs.push(c3.value),
            Diamonds => diamonds.push(c3.value),
            Hearts => hearts.push(c3.value),
            Spades => spades.push(c3.value),
        }

        let int_resuilt = [clubs.iter().fold(0, |acc, v| acc + v.score()),
                           diamonds.iter().fold(0, |acc, v| acc + v.score()),
                           hearts.iter().fold(0, |acc, v| acc + v.score()),
                           spades.iter().fold(0, |acc, v| acc + v.score())]
                .iter()
                .fold(0, |acc, x| std::cmp::max(acc, *x));

        Simple(int_resuilt)
    }
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub enum Score {
    ThirtyAndAHalf,
    Simple(u8),
}
use Score::*;

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} points", match *self {
            ThirtyAndAHalf => "30½".to_string(),
            Simple(x) => x.to_string(),
        })
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Score) -> Ordering {
        match (self, other) {
            (&ThirtyAndAHalf, &ThirtyAndAHalf) => Equal,
            (&ThirtyAndAHalf, &Simple(x)) => if x > 30 { Less } else { Greater },
            (&Simple(x), &ThirtyAndAHalf) => if x > 30 { Greater } else { Less },
            (&Simple(x1), &Simple(x2)) => x1.cmp(&x2),
        }
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Score) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone)]
pub enum HandCard {
    FirstCard,
    SecondCard,
    ThirdCard,
}
use HandCard::*;

#[derive(Eq, Clone)]
pub struct Card {
    pub suit: Suit,
    pub value: Value,
}

impl AllValues for Card {
    fn all_values() -> Vec<Card> {
        let mut deck = Vec::new();

        for &suit in Suit::all_values().iter() {
            for &value in Value::all_values().iter() {
                deck.push(Card {
                              suit: suit,
                              value: value,
                          });
            }
        }

        deck
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Card) -> Ordering {
        match self.suit.cmp(&other.suit) {
            Equal => self.value.cmp(&other.value),
            otherwise => otherwise,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Card) -> bool {
        self.suit == other.suit && self.value == other.value
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}", self.value, self.suit)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}
use Suit::*;

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Clubs => "♣".to_string(),
            Diamonds => "♦".to_string(),
            Hearts => "♥".to_string(),
            Spades => "♠".to_string(),
        })
    }
}

impl AllValues for Suit {
    fn all_values() -> Vec<Suit> {
        vec![Clubs, Diamonds, Hearts, Spades]
    }
}

impl Ord for Suit {
    fn cmp(&self, other: &Suit) -> Ordering {
        u8::from(*self).cmp(&u8::from(*other))
    }
}

impl From<Suit> for u8 {
    fn from(suit: Suit) -> Self {
        match suit {
            Clubs => 1,
            Diamonds => 2,
            Hearts => 3,
            Spades => 4,
        }
    }
}

impl PartialOrd for Suit {
    fn partial_cmp(&self, other: &Suit) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Value {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}
use Value::*;

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Ace => "A".to_string(),
            Two => "2".to_string(),
            Three => "3".to_string(),
            Four => "4".to_string(),
            Five => "5".to_string(),
            Six => "6".to_string(),
            Seven => "7".to_string(),
            Eight => "8".to_string(),
            Nine => "9".to_string(),
            Ten => "10".to_string(),
            Jack => "J".to_string(),
            Queen => "Q".to_string(),
            King => "K".to_string(),
        })
    }
}

impl AllValues for Value {
    fn all_values() -> Vec<Value> {
        vec![Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King]
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Value) -> Ordering {
        u8::from(*self).cmp(&u8::from(*other))
    }
}

impl From<Value> for u8 {
    fn from(value: Value) -> Self {
        match value {
            Ace => 14, //Ace high
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5,
            Six => 6,
            Seven => 7,
            Eight => 8,
            Nine => 9,
            Ten => 10,
            Jack => 11,
            Queen => 12,
            King => 13,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Value {
    fn score(&self) -> u8 {
        match *self {
            Ace => 11, //Ace high
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5,
            Six => 6,
            Seven => 7,
            Eight => 8,
            Nine => 9,
            Ten => 10,
            Jack => 10,
            Queen => 10,
            King => 10,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum KnockerOrWinner {
    Knocker(Participant),
    Winner(Participant),
}







//NOTE(Ryan1729): if I import BearLibTerminal.rs into `state_manipulation` or a crate
//`state_manipulation` depends on, like this one for example, then the
//ffi to the C version of BearLibTerminal causes an error. I just want
//the geometry datatypes and the Event and Keycode definitions so I have
//copied them from BearLibTerminal.rs below

//BearLibTerminal.rs is released under the MIT license by nabijaczleweli.
//see https://github.com/nabijaczleweli/BearLibTerminal.rs/blob/master/LICENSE
//for full details.

impl Point {
    /// Creates a new point on the specified non-negative coordinates
    pub fn new_safe(mut x: i32, mut y: i32) -> Point {
        x = if x >= 0 { x } else { 0 };
        y = if y >= 0 { y } else { 0 };

        Point { x: x, y: y }
    }

    pub fn add(&self, x: i32, y: i32) -> Point {
        Point::new_safe(self.x + x, self.y + y)
    }
}

/// Represents a single on-screen point/coordinate pair.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// Creates a new point on the specified non-negative coordinates
    pub fn new(x: i32, y: i32) -> Point {
        assert!(x >= 0);
        assert!(y >= 0);

        Point { x: x, y: y }
    }
}


/// A 2D size representation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

impl Size {
    /// Creates a new non-negative size.
    pub fn new(width: i32, height: i32) -> Size {
        assert!(width >= 0);
        assert!(height >= 0);

        Size {
            width: width,
            height: height,
        }
    }
}

impl fmt::Display for Size {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}x{}", self.width, self.height)
    }
}

/// A rectangle, described by its four corners and a size.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Rect {
    /// The top-left corner.
    pub top_left: Point,
    /// The top-right corner.
    pub top_right: Point,
    /// The bottom-right corner.
    pub bottom_right: Point,
    /// The bottom-left corner.
    pub bottom_left: Point,
    /// The `Rect`angle's size.
    pub size: Size,
}

impl Rect {
    /// Construct a `Rect` from its top-left corner and its size.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bear_lib_terminal::geometry::{Rect, Point, Size};
    /// let rect = Rect::from_size(Point::new(10, 20), Size::new(30, 40));
    /// assert_eq!(rect.top_left, Point::new(10, 20));
    /// assert_eq!(rect.top_right, Point::new(40, 20));
    /// assert_eq!(rect.bottom_left, Point::new(10, 60));
    /// assert_eq!(rect.bottom_right, Point::new(40, 60));
    /// assert_eq!(rect.size, Size::new(30, 40));
    /// ```
    pub fn from_size(origin: Point, size: Size) -> Rect {
        let top_right = Point::new(origin.x + size.width, origin.y);
        let bottom_left = Point::new(origin.x, origin.y + size.height);
        let bottom_right = Point::new(top_right.x, bottom_left.y);

        Rect {
            top_left: origin,
            top_right: top_right,
            bottom_left: bottom_left,
            bottom_right: bottom_right,
            size: size,
        }
    }

    /// Construct a `Rect` from its top-left and bottom-right corners.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bear_lib_terminal::geometry::{Rect, Point, Size};
    /// let rect = Rect::from_points(Point::new(10, 20), Point::new(30, 40));
    /// assert_eq!(rect.top_left, Point::new(10, 20));
    /// assert_eq!(rect.top_right, Point::new(30, 20));
    /// assert_eq!(rect.bottom_left, Point::new(10, 40));
    /// assert_eq!(rect.bottom_right, Point::new(30, 40));
    /// assert_eq!(rect.size, Size::new(20, 20));
    /// ```
    pub fn from_points(top_left: Point, bottom_right: Point) -> Rect {
        assert!(bottom_right.x >= top_left.x);
        assert!(bottom_right.y >= top_left.y);

        let size = Size::new(bottom_right.x - top_left.x, bottom_right.y - top_left.y);
        Rect::from_size(top_left, size)
    }

    /// Construct a `Rect` from its top-left corner and its size, values unwrapped.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bear_lib_terminal::geometry::{Rect, Point, Size};
    /// assert_eq!(Rect::from_values(10, 20, 30, 40),
    ///     Rect::from_size(Point::new(10, 20), Size::new(30, 40)));
    /// ```
    pub fn from_values(x: i32, y: i32, width: i32, height: i32) -> Rect {
        let origin = Point::new(x, y);
        let size = Size::new(width, height);
        Rect::from_size(origin, size)
    }


    /// Construct a `Rect` from its top-left and bottom-right corners, values unwrapped.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bear_lib_terminal::geometry::{Rect, Point, Size};
    /// assert_eq!(Rect::from_point_values(10, 20, 30, 40),
    ///     Rect::from_points(Point::new(10, 20), Point::new(30, 40)));
    /// ```
    pub fn from_point_values(top_left_x: i32,
                             top_left_y: i32,
                             bottom_right_x: i32,
                             bottom_right_y: i32)
                             -> Rect {
        let top_left = Point::new(top_left_x, top_left_y);
        let bottom_right = Point::new(bottom_right_x, bottom_right_y);
        Rect::from_points(top_left, bottom_right)
    }
}

//input module

/// All pressable keys.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum KeyCode {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    /// Top-row `1/!` key.
    Row1,
    /// Top-row `2/@` key.
    Row2,
    /// Top-row `3/#` key.
    Row3,
    /// Top-row `4/$` key.
    Row4,
    /// Top-row `5/%` key.
    Row5,
    /// Top-row `6/^` key.
    Row6,
    /// Top-row `7/&` key.
    Row7,
    /// Top-row `8/*` key.
    Row8,
    /// Top-row `9/(` key.
    Row9,
    /// Top-row `0/)` key.
    Row0,
    /// Top-row &#96;/~ key.
    Grave,
    /// Top-row `-/_` key.
    Minus,
    /// Top-row `=/+` key.
    Equals,
    /// Second-row `[/{` key.
    LeftBracket,
    /// Second-row `]/}` key.
    RightBracket,
    /// Second-row `\/|` key.
    Backslash,
    /// Third-row `;/:` key.
    Semicolon,
    /// Third-row `'/"` key.
    Apostrophe,
    /// Fourth-row `,/<` key.
    Comma,
    /// Fourth-row `./>` key.
    Period,
    /// Fourth-row `//?` key.
    Slash,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Enter,
    Escape,
    Backspace,
    Tab,
    Space,
    Pause,
    Insert,
    Home,
    PageUp,
    Delete,
    End,
    PageDown,
    /// Right arrow key.
    Right,
    /// Left arrow key.
    Left,
    /// Down arrow key.
    Down,
    /// Up arrow key.
    Up,
    /// Numpad `/` key.
    NumDivide,
    /// Numpad `*` key.
    NumMultiply,
    /// Numpad `-` key.
    NumMinus,
    /// Numpad `+` key.
    NumPlus,
    /// Numpad &#9166; key.
    NumEnter,
    /// Numpad `Del/.` key (output locale-dependent).
    NumPeriod,
    /// Numpad `1/End` key.
    Num1,
    /// Numpad 2/&#8595; key.
    Num2,
    /// Numpad `3/PageDown` key.
    Num3,
    /// Numpad 4/&#8592; key.
    Num4,
    /// Numpad `5` key.
    Num5,
    /// Numpad 6/&#8594; key.
    Num6,
    /// Numpad `7/Home` key.
    Num7,
    /// Numpad 8/&#8593; key.
    Num8,
    /// Numpad `9/PageUp` key.
    Num9,
    /// Numpad `0/Insert` key.
    Num0,
    /// Left mouse button.
    MouseLeft,
    /// Right mouse button.
    MouseRight,
    /// Middle mouse button a.k.a. pressed scroll wheel.
    MouseMiddle,
    MouseFourth,
    MouseFifth,
}

/// A single input event.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Event {
    /// Terminal window closed.
    Close,
    /// Terminal window resized. Needs to have `window.resizeable = true` to occur.
    ///
    /// Note, that the terminal window is cleared when resized.
    Resize {
        /// Width the terminal was resized to.
        width: i32,
        /// Heigth the terminal was resized to.
        height: i32,
    },
    /// Mouse moved.
    ///
    /// If [`precise-mouse`](config/struct.Input.html#structfield.precise_mouse) is off,
    /// generated each time mouse moves from cell to cell, otherwise,
    /// when it moves from pixel to pixel.
    MouseMove {
        /// `0`-based cell index from the left to which the mouse cursor moved.
        x: i32,
        /// `0`-based cell index from the top to which the mouse cursor moved.
        y: i32,
    },
    /// Mouse wheel moved.
    MouseScroll {
        /// Amount of steps the wheel rotated.
        ///
        /// Positive when scrolled "down"/"backwards".
        ///
        /// Negative when scrolled "up"/"forwards"/"away".
        delta: i32,
    },
    /// A keyboard or mouse button pressed (might repeat, if set in OS).
    KeyPressed {
        /// The key pressed.
        key: KeyCode,
        /// Whether the Control key is pressed.
        ctrl: bool,
        /// Whether the Shift key is pressed.
        shift: bool,
    },
    /// A keyboard or mouse button released.
    KeyReleased {
        /// The key released.
        key: KeyCode,
        /// Whether the Control key is pressed.
        ctrl: bool,
        /// Whether the Shift key is pressed.
        shift: bool,
    },
    /// The Shift key pressed (might repeat, if set in OS).
    ShiftPressed,
    /// The Shift key released.
    ShiftReleased,
    /// The Shift key pressed (might repeat, if set in OS).
    ControlPressed,
    /// The Control key released.
    ControlReleased,
}

pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}
