use crate::{one_line, Point2, Range2};
use nom::IResult;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TileSet {
    tiles: HashSet<Point2>,
    active_char: char,
}

impl TileSet {
    pub fn new() -> Self {
        Self {
            tiles: HashSet::new(),
            active_char: '#',
        }
    }

    pub fn with_active_char(self, active_char: char) -> Self {
        Self {
            active_char,
            ..self
        }
    }

    pub fn with_tiles<I>(self, tiles: I) -> Self
    where
        I: IntoIterator<Item = Point2>,
    {
        Self {
            tiles: tiles.into_iter().collect(),
            ..self
        }
    }

    pub fn from_string<const ACTIVE_CHAR: char>(input: &str) -> Self {
        Self::parser::<ACTIVE_CHAR>()(input).unwrap().1
    }

    pub fn parser<const ACTIVE_CHAR: char>() -> impl Fn(&str) -> IResult<&str, Self> {
        move |mut input: &str| {
            input = input.trim_start();
            let mut tiles = HashSet::new();
            let mut y = 0;
            loop {
                let line: &str;
                (input, line) = one_line(input)?;
                if line.is_empty() {
                    break;
                }

                for (x, c) in line.chars().enumerate() {
                    if c == ACTIVE_CHAR {
                        let p = (x as i32, y).into();
                        tiles.insert(p);
                    }
                }
                y += 1;
            }
            Ok((
                input,
                Self {
                    tiles,
                    active_char: ACTIVE_CHAR,
                },
            ))
        }
    }

    pub fn get_range(&self) -> Option<Range2> {
        Point2::get_range(&self.tiles)
    }
}

impl Default for TileSet {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for TileSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let range = self.get_range().unwrap();
        for y in range.y.0..=range.y.1 {
            for x in range.x.0..=range.x.1 {
                if self.tiles.contains(&Point2 { x, y }) {
                    write!(f, "{}", self.active_char)?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl std::ops::Deref for TileSet {
    type Target = HashSet<Point2>;
    fn deref(&self) -> &Self::Target {
        &self.tiles
    }
}

impl std::ops::DerefMut for TileSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tiles
    }
}

pub trait TileChar
where
    Self: Sized,
{
    fn to_char(&self) -> char;
    fn from_char(c: char) -> Option<Self>;
    fn all_chars() -> Vec<char>;
}

const DEFAULT_BACKGROUND_CHAR: char = '.';

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TileMap<T, const B: char = DEFAULT_BACKGROUND_CHAR> {
    tiles: HashMap<Point2, T>,
}

impl<T: TileChar, const B: char> TileMap<T, B> {
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
        }
    }

    pub fn with_tiles<I>(self, tiles: I) -> Self
    where
        I: IntoIterator<Item = (Point2, T)>,
    {
        Self {
            tiles: tiles.into_iter().collect(),
        }
    }

    pub fn from_string(input: &str) -> Self {
        Self::parser(input).unwrap().1
    }

    pub fn parser(input: &str) -> IResult<&str, Self> {
        let (pixels, input) = if let Some(idx) = input.find("\n\n") {
            input.split_at(idx)
        } else {
            (input, "")
        };

        let mut tiles = HashMap::new();
        for (y, line) in pixels.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if let Some(t) = T::from_char(c) {
                    let p = (x as i32, y as i32).into();
                    tiles.insert(p, t);
                }
            }
        }
        Ok((input, Self { tiles }))
    }

    pub fn get_range(&self) -> Option<Range2> {
        Point2::get_range(self.tiles.keys())
    }
}

impl<T: TileChar, const B: char> Default for TileMap<T, B> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: TileChar, const B: char> std::fmt::Display for TileMap<T, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let range = self.get_range().unwrap();
        for y in range.y.0..=range.y.1 {
            for x in range.x.0..=range.x.1 {
                if let Some(t) = self.tiles.get(&Point2 { x, y }) {
                    write!(f, "{}", t.to_char())?;
                } else {
                    write!(f, "{}", B)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: TileChar, const B: char> std::ops::Deref for TileMap<T, B> {
    type Target = HashMap<Point2, T>;
    fn deref(&self) -> &Self::Target {
        &self.tiles
    }
}

impl<T: TileChar, const B: char> std::ops::DerefMut for TileMap<T, B> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tiles
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tileset_from_string() {
        let input = ".";
        let tileset = TileSet::from_string::<'#'>(input);
        assert_eq!(
            tileset,
            TileSet {
                tiles: HashSet::new(),
                active_char: '#'
            }
        );

        let input = "\
###
#.#
###";
        let tileset = TileSet::from_string::<'#'>(input);
        let expected: HashSet<Point2> = [
            (0, 0),
            (1, 0),
            (2, 0),
            (0, 1),
            (2, 1),
            (0, 2),
            (1, 2),
            (2, 2),
        ]
        .into_iter()
        .map(|t| t.into())
        .collect();
        assert_eq!(
            tileset,
            TileSet {
                tiles: expected,
                active_char: '#'
            }
        );

        let input = "\
X..
.X.
..X";
        let tileset = TileSet::from_string::<'X'>(input);
        let expected: HashSet<Point2> =
            [(0, 0), (1, 1), (2, 2)].iter().map(|&t| t.into()).collect();
        assert_eq!(
            tileset,
            TileSet {
                tiles: expected,
                active_char: 'X'
            }
        );

        let input = "\
X.
.X

.X
X.";
        let tileset = TileSet::from_string::<'X'>(input);
        let expected: HashSet<Point2> = [(0, 0), (1, 1)].iter().map(|&t| t.into()).collect();
        assert_eq!(
            tileset,
            TileSet {
                tiles: expected,
                active_char: 'X'
            }
        );
    }

    #[test]
    fn test_tileset_deref() {
        let input = "\
X..
.X.
..X";
        let tileset = TileSet::from_string::<'X'>(input);
        assert_eq!(tileset.contains(&(0, 0).into()), true);
        assert_eq!(tileset.contains(&(1, 1).into()), true);
        assert_eq!(tileset.contains(&(2, 2).into()), true);
        assert_eq!(tileset.contains(&(3, 3).into()), false);
        assert_eq!(tileset.contains(&(-1, -1).into()), false);
    }

    #[test]
    fn test_tileset_display() {
        let input = "\
###
#.#
###";
        let tileset = TileSet::from_string::<'#'>(input);
        assert_eq!(tileset.to_string().trim(), input);

        let input = "\
X..
.X.
..X";
        let tileset = TileSet::from_string::<'X'>(input);
        assert_eq!(tileset.to_string().trim(), input);
    }

    #[test]
    fn test_tileset_range() {
        let input = ".";
        let tileset = TileSet::from_string::<'#'>(input);
        assert_eq!(tileset.get_range(), None);

        let input = "\
###
#.#
###";
        let tileset = TileSet::from_string::<'#'>(input);
        assert_eq!(
            tileset.get_range(),
            Some(Range2 {
                x: (0, 2),
                y: (0, 2)
            })
        );

        let input = "#.#.#.#.#.#.#.#.#";
        let tileset = TileSet::from_string::<'#'>(input);
        assert_eq!(
            tileset.get_range(),
            Some(Range2 {
                x: (0, 16),
                y: (0, 0)
            })
        );
    }

    #[derive(Debug, PartialEq)]
    enum MyTile {
        A,
        B,
        C,
    }

    impl TileChar for MyTile {
        fn to_char(&self) -> char {
            match self {
                Self::A => 'A',
                Self::B => 'B',
                Self::C => 'C',
            }
        }

        fn from_char(c: char) -> Option<Self> {
            Some(match c {
                'A' => Self::A,
                'B' => Self::B,
                'C' => Self::C,
                _ => return None,
            })
        }

        fn all_chars() -> Vec<char> {
            vec!['A', 'B', 'C']
        }
    }

    #[test]
    fn test_tilemap_from_string() {
        let input = ".";
        let tilemap: TileMap<MyTile> = TileMap::from_string(input);
        assert_eq!(
            tilemap,
            TileMap {
                tiles: HashMap::new()
            }
        );

        let input = "\
ABC
A.A
CBA";
        let tilemap: TileMap<MyTile> = TileMap::from_string(input);
        let expected: HashMap<Point2, MyTile> = [
            ((0, 0), MyTile::A),
            ((1, 0), MyTile::B),
            ((2, 0), MyTile::C),
            ((0, 1), MyTile::A),
            ((2, 1), MyTile::A),
            ((0, 2), MyTile::C),
            ((1, 2), MyTile::B),
            ((2, 2), MyTile::A),
        ]
        .into_iter()
        .map(|(p, t)| (p.into(), t))
        .collect();
        assert_eq!(tilemap, TileMap { tiles: expected });

        let input = "\
AA
AA

BB
BB";
        let tilemap: TileMap<MyTile> = TileMap::from_string(input);
        let expected: HashMap<Point2, MyTile> = [
            ((0, 0), MyTile::A),
            ((1, 0), MyTile::A),
            ((0, 1), MyTile::A),
            ((1, 1), MyTile::A),
        ]
        .into_iter()
        .map(|(p, t)| (p.into(), t))
        .collect();
        assert_eq!(tilemap, TileMap { tiles: expected });
    }

    #[test]
    fn test_tilemap_deref() {
        let input = "\
A..
.B.
..C";
        let tilemap: TileMap<MyTile> = TileMap::from_string(input);
        assert_eq!(tilemap.get(&(0, 0).into()).is_some(), true);
        assert_eq!(tilemap.get(&(1, 1).into()).is_some(), true);
        assert_eq!(tilemap.get(&(2, 2).into()).is_some(), true);
        assert_eq!(tilemap.get(&(3, 3).into()).is_some(), false);
        assert_eq!(tilemap.get(&(-1, -1).into()).is_some(), false);
    }

    #[test]
    fn test_tilemap_display() {
        let input = "\
ABC
A.A
CBA";
        let tilemap: TileMap<MyTile> = TileMap::from_string(input);
        assert_eq!(tilemap.to_string().trim(), input);

        let input = "\
A..
.B.
..C";
        let tilemap: TileMap<MyTile> = TileMap::from_string(input);
        assert_eq!(tilemap.to_string().trim(), input);
    }

    #[test]
    fn test_tilemap_range() {
        let input = ".";
        let tilemap: TileMap<MyTile> = TileMap::from_string(input);
        assert_eq!(tilemap.get_range(), None);

        let input = "\
ABC
A.A
CBA";
        let tilemap: TileMap<MyTile> = TileMap::from_string(input);
        assert_eq!(
            tilemap.get_range(),
            Some(Range2 {
                x: (0, 2),
                y: (0, 2)
            })
        );

        let input = "ABCABCABCABCABC";
        let tilemap: TileMap<MyTile> = TileMap::from_string(input);
        assert_eq!(
            tilemap.get_range(),
            Some(Range2 {
                x: (0, 14),
                y: (0, 0)
            })
        );
    }
}
