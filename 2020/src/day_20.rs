/*
    --- Day 20: Jurassic Jigsaw ---
    The high-speed train leaves the forest and quickly carries you south. You can even see a desert in the distance! Since you have some spare time, you might as well see if there was anything interesting in the image the Mythical Information Bureau satellite captured.

    After decoding the satellite messages, you discover that the data actually contains many small images created by the satellite's camera array. The camera array consists of many cameras; rather than produce a single square image, they produce many smaller square image tiles that need to be reassembled back into a single image.

    Each camera in the camera array returns a single monochrome image tile with a random unique ID number. The tiles (your puzzle input) arrived in a random order.

    Worse yet, the camera array appears to be malfunctioning: each image tile has been rotated and flipped to a random orientation. Your first task is to reassemble the original image by orienting the tiles so they fit together.

    To show how the tiles should be reassembled, each tile's image data includes a border that should line up exactly with its adjacent tiles. All tiles have this border, and the border lines up exactly when the tiles are both oriented correctly. Tiles at the edge of the image also have this border, but the outermost edges won't line up with any other tiles.

    For example, suppose you have the following nine tiles:

    Tile 2311:
    ..##.#..#.
    ##..#.....
    #...##..#.
    ####.#...#
    ##.##.###.
    ##...#.###
    .#.#.#..##
    ..#....#..
    ###...#.#.
    ..###..###

    Tile 1951:
    #.##...##.
    #.####...#
    .....#..##
    #...######
    .##.#....#
    .###.#####
    ###.##.##.
    .###....#.
    ..#.#..#.#
    #...##.#..

    Tile 1171:
    ####...##.
    #..##.#..#
    ##.#..#.#.
    .###.####.
    ..###.####
    .##....##.
    .#...####.
    #.##.####.
    ####..#...
    .....##...

    Tile 1427:
    ###.##.#..
    .#..#.##..
    .#.##.#..#
    #.#.#.##.#
    ....#...##
    ...##..##.
    ...#.#####
    .#.####.#.
    ..#..###.#
    ..##.#..#.

    Tile 1489:
    ##.#.#....
    ..##...#..
    .##..##...
    ..#...#...
    #####...#.
    #..#.#.#.#
    ...#.#.#..
    ##.#...##.
    ..##.##.##
    ###.##.#..

    Tile 2473:
    #....####.
    #..#.##...
    #.##..#...
    ######.#.#
    .#...#.#.#
    .#########
    .###.#..#.
    ########.#
    ##...##.#.
    ..###.#.#.

    Tile 2971:
    ..#.#....#
    #...###...
    #.#.###...
    ##.##..#..
    .#####..##
    .#..####.#
    #..#.#..#.
    ..####.###
    ..#.#.###.
    ...#.#.#.#

    Tile 2729:
    ...#.#.#.#
    ####.#....
    ..#.#.....
    ....#..#.#
    .##..##.#.
    .#.####...
    ####.#.#..
    ##.####...
    ##..#.##..
    #.##...##.

    Tile 3079:
    #.#.#####.
    .#..######
    ..#.......
    ######....
    ####.#..#.
    .#...#.##.
    #.#####.##
    ..#.###...
    ..#.......
    ..#.###...
    By rotating, flipping, and rearranging them, you can find a square arrangement that causes all adjacent borders to line up:

    #...##.#.. ..###..### #.#.#####.
    ..#.#..#.# ###...#.#. .#..######
    .###....#. ..#....#.. ..#.......
    ###.##.##. .#.#.#..## ######....
    .###.##### ##...#.### ####.#..#.
    .##.#....# ##.##.###. .#...#.##.
    #...###### ####.#...# #.#####.##
    .....#..## #...##..#. ..#.###...
    #.####...# ##..#..... ..#.......
    #.##...##. ..##.#..#. ..#.###...

    #.##...##. ..##.#..#. ..#.###...
    ##..#.##.. ..#..###.# ##.##....#
    ##.####... .#.####.#. ..#.###..#
    ####.#.#.. ...#.##### ###.#..###
    .#.####... ...##..##. .######.##
    .##..##.#. ....#...## #.#.#.#...
    ....#..#.# #.#.#.##.# #.###.###.
    ..#.#..... .#.##.#..# #.###.##..
    ####.#.... .#..#.##.. .######...
    ...#.#.#.# ###.##.#.. .##...####

    ...#.#.#.# ###.##.#.. .##...####
    ..#.#.###. ..##.##.## #..#.##..#
    ..####.### ##.#...##. .#.#..#.##
    #..#.#..#. ...#.#.#.. .####.###.
    .#..####.# #..#.#.#.# ####.###..
    .#####..## #####...#. .##....##.
    ##.##..#.. ..#...#... .####...#.
    #.#.###... .##..##... .####.##.#
    #...###... ..##...#.. ...#..####
    ..#.#....# ##.#.#.... ...##.....
    For reference, the IDs of the above tiles are:

    1951    2311    3079
    2729    1427    2473
    2971    1489    1171
    To check that you've assembled the image correctly, multiply the IDs of the four corner tiles together. If you do this with the assembled tiles from the example above, you get 1951 * 3079 * 2971 * 1171 = 20899048083289.

    Assemble the tiles into an image. What do you get if you multiply together the IDs of the four corner tiles?

    --- Part Two ---
    Now, you're ready to check the image for sea monsters.

    The borders of each tile are not part of the actual image; start by removing them.

    In the example above, the tiles become:

    .#.#..#. ##...#.# #..#####
    ###....# .#....#. .#......
    ##.##.## #.#.#..# #####...
    ###.#### #...#.## ###.#..#
    ##.#.... #.##.### #...#.##
    ...##### ###.#... .#####.#
    ....#..# ...##..# .#.###..
    .####... #..#.... .#......

    #..#.##. .#..###. #.##....
    #.####.. #.####.# .#.###..
    ###.#.#. ..#.#### ##.#..##
    #.####.. ..##..## ######.#
    ##..##.# ...#...# .#.#.#..
    ...#..#. .#.#.##. .###.###
    .#.#.... #.##.#.. .###.##.
    ###.#... #..#.##. ######..

    .#.#.### .##.##.# ..#.##..
    .####.## #.#...## #.#..#.#
    ..#.#..# ..#.#.#. ####.###
    #..####. ..#.#.#. ###.###.
    #####..# ####...# ##....##
    #.##..#. .#...#.. ####...#
    .#.###.. ##..##.. ####.##.
    ...###.. .##...#. ..#..###
    Remove the gaps to form the actual image:

    .#.#..#.##...#.##..#####
    ###....#.#....#..#......
    ##.##.###.#.#..######...
    ###.#####...#.#####.#..#
    ##.#....#.##.####...#.##
    ...########.#....#####.#
    ....#..#...##..#.#.###..
    .####...#..#.....#......
    #..#.##..#..###.#.##....
    #.####..#.####.#.#.###..
    ###.#.#...#.######.#..##
    #.####....##..########.#
    ##..##.#...#...#.#.#.#..
    ...#..#..#.#.##..###.###
    .#.#....#.##.#...###.##.
    ###.#...#..#.##.######..
    .#.#.###.##.##.#..#.##..
    .####.###.#...###.#..#.#
    ..#.#..#..#.#.#.####.###
    #..####...#.#.#.###.###.
    #####..#####...###....##
    #.##..#..#...#..####...#
    .#.###..##..##..####.##.
    ...###...##...#...#..###
    Now, you're ready to search for sea monsters! Because your image is monochrome, a sea monster will look like this:

                    #
    #    ##    ##    ###
    #  #  #  #  #  #
    When looking for this pattern in the image, the spaces can be anything; only the # need to match. Also, you might need to rotate or flip your image before it's oriented correctly to find sea monsters. In the above image, after flipping and rotating it to the appropriate orientation, there are two sea monsters (marked with O):

    .####...#####..#...###..
    #####..#..#.#.####..#.#.
    .#.#...#.###...#.##.O#..
    #.O.##.OO#.#.OO.##.OOO##
    ..#O.#O#.O##O..O.#O##.##
    ...#.#..##.##...#..#..##
    #.##.#..#.#..#..##.#.#..
    .###.##.....#...###.#...
    #.####.#.#....##.#..#.#.
    ##...#..#....#..#...####
    ..#.##...###..#.#####..#
    ....#.##.#.#####....#...
    ..##.##.###.....#.##..#.
    #...#...###..####....##.
    .#.##...#.##.#.#.###...#
    #.###.#..####...##..#...
    #.###...#.##...#.##O###.
    .O##.#OO.###OO##..OOO##.
    ..O#.O..O..O.#O##O##.###
    #.#..##.########..#..##.
    #.#####..#.#...##..#....
    #....##..#.#########..##
    #...#.....#..##...###.##
    #..###....##.#...##.##.#
    Determine how rough the waters are in the sea monsters' habitat by counting the number of # that are not part of a sea monster. In the above example, the habitat's water roughness is 273.

    How many # are not part of a sea monster?
*/

use crate::common::{modulo, trim_start, unsigned, Point, TileChar, TileMap, TileSet};
use nom::{
    bytes::complete::tag,
    character::complete::char,
    multi::many1,
    sequence::{delimited, pair},
    IResult,
};
use std::collections::HashMap;

const TILE_SIZE: usize = 10;
const MAX_ROT: u32 = 4;

fn bit_reverse(input: &u32, size: usize) -> u32 {
    assert!(size > 0);

    let mut output = 0;
    for i in 0..size {
        let in_bit = (input & (1 << i)) != 0;
        if in_bit == true {
            output |= 1 << (size - 1 - i);
        }
    }
    output
}

fn transform(pixels: &TileSet, orientation: TileOrientation) -> TileSet {
    assert!(orientation.rotation < MAX_ROT);
    let (mut x_range, mut y_range) = pixels.get_range().unwrap();
    let mut output = TileSet::new();
    for p in pixels.iter() {
        let mut p_new = *p;
        if orientation.flipped == true {
            p_new.x = x_range.1 - (p_new.x - x_range.0);
        }

        for _ in 0..orientation.rotation {
            let tmp = p_new.x;
            p_new.x = x_range.0 + (y_range.1 - p_new.y);
            p_new.y = y_range.0 + (tmp - x_range.0);

            // Swap the ranges too since we're not zero-centered
            let tmp = x_range;
            x_range = (x_range.0, x_range.0 + y_range.1 - y_range.0);
            y_range = (y_range.0, y_range.0 + tmp.1 - tmp.0);
        }

        output.insert(p_new);
    }
    output
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum TileSide {
    Top = 0,
    Right = 1,
    Bottom = 2,
    Left = 3,
}

impl TileSide {
    fn to_unit_point(&self) -> Point {
        match self {
            Self::Top => (0, -1),
            Self::Right => (1, 0),
            Self::Bottom => (0, 1),
            Self::Left => (-1, 0),
        }
        .into()
    }

    fn opposite(&self) -> Self {
        match self {
            Self::Top => Self::Bottom,
            Self::Right => Self::Left,
            Self::Bottom => Self::Top,
            Self::Left => Self::Right,
        }
    }

    const VALUES: [Self; 4] = [Self::Top, Self::Right, Self::Bottom, Self::Left];

    fn iter() -> impl Iterator<Item = Self> {
        Self::VALUES.iter().copied()
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
struct TileOrientation {
    rotation: u32,
    flipped: bool,
}

#[derive(Clone, Debug)]
pub struct ImageTile {
    id: u64,
    pixels: TileSet,
    side_a_ids: [u32; 4],
    side_b_ids: [u32; 4],
}

impl ImageTile {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (id, pixels)) = pair(
            delimited(trim_start(tag("Tile ")), unsigned, char(':')),
            TileSet::parser('#'),
        )(input)?;

        let (x_range, y_range) = pixels.get_range().unwrap();
        fn make_side_id<I>(pixels: &TileSet, values: I) -> u32
        where
            I: Iterator<Item = Point>,
        {
            values.fold(0, |mut acc, p| {
                acc <<= 1;
                if pixels.contains(&p) == true {
                    acc |= 1;
                }
                acc
            })
        };
        let top_a = make_side_id(
            &pixels,
            (x_range.0..=x_range.1).map(|x| Point { x, y: y_range.0 }),
        );
        let bottom_a = make_side_id(
            &pixels,
            (x_range.0..=x_range.1).map(|x| Point { x, y: y_range.1 }),
        );
        let left_a = make_side_id(
            &pixels,
            (y_range.0..=y_range.1).map(|y| Point { x: x_range.0, y }),
        );
        let right_a = make_side_id(
            &pixels,
            (y_range.0..=y_range.1).map(|y| Point { x: x_range.1, y }),
        );

        // Flip horizontally; top and bottom are reversed bitwise, left and right swap
        let top_b = bit_reverse(&top_a, TILE_SIZE);
        let bottom_b = bit_reverse(&bottom_a, TILE_SIZE);
        let left_b = right_a;
        let right_b = left_a;

        Ok((
            input,
            Self {
                id,
                pixels,
                side_a_ids: [top_a, right_a, bottom_a, left_a],
                side_b_ids: [top_b, right_b, bottom_b, left_b],
            },
        ))
    }

    fn get_side_id(&self, side: TileSide, orientation: TileOrientation) -> u32 {
        // Side IDs are encoded left to right and top to bottom. So, when rotating,
        // the ID will flip sometimes - for example rotating 180 degrees means all
        // sides are now reverse of their original encoding.
        let rot_idx = modulo(MAX_ROT - modulo(orientation.rotation, MAX_ROT), MAX_ROT);
        let idx = modulo(side as u32 + rot_idx, MAX_ROT) as usize;

        let ary = if orientation.flipped == false {
            &self.side_a_ids
        } else {
            &self.side_b_ids
        };

        let side_rev = (side as u32) >= 2;
        let idx_rev = idx >= 2;
        if side_rev ^ idx_rev {
            // Reverse the bits if starting in top or right and moving to bottom or left, or vice versa
            bit_reverse(&ary[idx], TILE_SIZE)
        } else {
            ary[idx]
        }
    }

    fn all_side_ids(&self) -> Vec<u32> {
        Self::all_orientations()
            .iter()
            .map(|&(s, o)| self.get_side_id(s, o))
            .collect()
    }

    fn all_orientations() -> Vec<(TileSide, TileOrientation)> {
        let mut output = Vec::new();
        for side in TileSide::iter() {
            for rotation in 0..MAX_ROT {
                for &flipped in &[false, true] {
                    output.push((side, TileOrientation { rotation, flipped }));
                }
            }
        }
        output
    }
}

impl std::fmt::Display for ImageTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x_range, y_range) = self.pixels.get_range().unwrap();
        for y in y_range.0..=y_range.1 {
            for x in x_range.0..=x_range.1 {
                if self.pixels.contains(&Point { x, y }) == true {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Image {
    tiles: HashMap<u64, ImageTile>,
    possibilities: HashMap<u32, Vec<(u64, TileSide, TileOrientation)>>,
    combined_pixels: TileSet,
}

impl Image {
    fn from_image_tiles(input: &[ImageTile]) -> Self {
        let tiles: HashMap<u64, ImageTile> = input.iter().map(|t| (t.id, t.clone())).collect();

        // Create a map of which other tiles could potentially connect to a given side ID.
        let mut possibilities = HashMap::new();
        for tile in tiles.values() {
            for (side, orientation) in ImageTile::all_orientations() {
                let side_id = tile.get_side_id(side, orientation);
                let entry = possibilities.entry(side_id).or_insert_with(Vec::new);
                entry.push((tile.id, side, orientation));
            }
        }

        Self {
            tiles,
            possibilities,
            combined_pixels: TileSet::new(),
        }
    }

    fn find_corners(&self) -> Vec<u64> {
        /*
            This function assumes the input data is well formed and is relatively simple. For example,
            tiles have at most one possibile connection for each side, and there are exactly four tiles
            that look like a corner (2 possible connections, both connections are adjacent, and this is
            true for both sides of the tile).
        */
        let mut corners: Vec<u64> = self
            .tiles
            .keys()
            .filter(|tile_id| {
                TileSide::iter()
                    .filter(|&side| {
                        let side_id =
                            self.tiles[&tile_id].get_side_id(side, TileOrientation::default());
                        let mut possible_tiles: Vec<u64> = self.possibilities[&side_id]
                            .iter()
                            .map(|&(t, ..)| t)
                            .collect();
                        possible_tiles.sort_unstable();
                        possible_tiles.dedup();
                        possible_tiles.len() == 1
                    })
                    .count()
                    == 2
            })
            .copied()
            .collect();
        corners.sort_unstable();
        corners.dedup();

        // Validate the result as best we can
        assert_eq!(corners.len(), 4);
        corners
    }

    fn assemble(&mut self) {
        // First get a list of corner pieces. The input data is simple enough that there should be exactly four.
        let corners: Vec<u64> = self.find_corners();

        // Place one corner, then build the image out from there. My input data has only one possibility for each side.
        fn place_tile(
            point: Point,
            tile_id: u64,
            orientation: TileOrientation,
            tile_map: &mut HashMap<Point, (u64, TileOrientation)>,
            unplaced_tiles: &mut Vec<u64>,
        ) {
            unplaced_tiles.remove(unplaced_tiles.iter().position(|x| *x == tile_id).unwrap());
            tile_map.insert(point, (tile_id, orientation));
        }

        let mut tile_map: HashMap<Point, (u64, TileOrientation)> = HashMap::new();
        let mut unplaced_tiles: Vec<u64> = self.tiles.keys().copied().collect();
        let mut frontier: Vec<Point> = Point::origin().orthogonals().collect();
        place_tile(
            Point::origin(),
            corners[0],
            TileOrientation::default(),
            &mut tile_map,
            &mut unplaced_tiles,
        );

        while let Some(next_point) = frontier.pop() {
            if tile_map.contains_key(&next_point) == true {
                continue;
            }

            // Get the candidate tiles; in practice there will be at most one!
            let mut candidates: Vec<(u64, TileSide, TileOrientation)> = Vec::new();
            for side in TileSide::iter() {
                if let Some(&(tile_id, orientation)) =
                    tile_map.get(&(next_point + side.to_unit_point()))
                {
                    let existing_side =
                        self.tiles[&tile_id].get_side_id(side.opposite(), orientation);
                    candidates.extend(self.possibilities[&existing_side].iter());
                }
            }
            candidates.retain(|&(t, ..)| unplaced_tiles.contains(&t));
            candidates.sort_unstable();
            candidates.dedup();
            if candidates.is_empty() == true {
                continue;
            }

            // Verify that there is only one candidate tile ID (there are likely multiple candidates with the same tile ID though)
            let mut tile_ids: Vec<u64> = candidates.iter().map(|(t, ..)| t).copied().collect();
            tile_ids.sort_unstable();
            tile_ids.dedup();
            assert_eq!(tile_ids.len(), 1);

            // Find the first candidate that meets all requirements
            let mut picked: Option<(u64, TileOrientation)> = None;
            for (candidate_tile_id, _, candidate_orientation) in candidates {
                let all_ok = TileSide::iter().all(|side| {
                    if let Some(&(tile_id, orientation)) =
                        tile_map.get(&(next_point + side.to_unit_point()))
                    {
                        let neighbor_side_id =
                            self.tiles[&tile_id].get_side_id(side.opposite(), orientation);
                        let candidate_side_id =
                            self.tiles[&candidate_tile_id].get_side_id(side, candidate_orientation);
                        neighbor_side_id == candidate_side_id
                    } else {
                        true
                    }
                });

                if all_ok == true {
                    picked = Some((candidate_tile_id, candidate_orientation));
                    break;
                }
            }

            let (picked_id, picked_orientation) = picked.unwrap();
            place_tile(
                next_point,
                picked_id,
                picked_orientation,
                &mut tile_map,
                &mut unplaced_tiles,
            );
            frontier.extend(next_point.orthogonals());
        }
        assert!(unplaced_tiles.is_empty());

        // Now create the image from each tile
        self.combined_pixels.clear();
        for (tile_point, (tile_id, tile_orientation)) in tile_map {
            let transformed_pixels = transform(&self.tiles[&tile_id].pixels, tile_orientation);
            let (x_range, y_range) = transformed_pixels.get_range().unwrap();
            let x_size = x_range.1 - x_range.0 - 1;
            let y_size = y_range.1 - y_range.0 - 1;
            let base_point = Point {
                x: tile_point.x * x_size,
                y: tile_point.y * y_size,
            };
            for p in transformed_pixels.iter() {
                // Strip off outer border
                if p.x != x_range.0 && p.x != x_range.1 && p.y != y_range.0 && p.y != y_range.1 {
                    self.combined_pixels.insert(base_point + p);
                }
            }
        }
    }

    fn find_sea_monsters(&self) -> usize {
        #[derive(PartialEq)]
        enum PixelTile {
            Empty,
            Wave,
            Monster,
        }

        impl TileChar for PixelTile {
            fn to_char(&self) -> char {
                match self {
                    Self::Empty => '.',
                    Self::Wave => '#',
                    Self::Monster => 'O',
                }
            }

            fn from_char(c: char) -> Option<Self> {
                Some(match c {
                    '.' => Self::Empty,
                    '#' => Self::Wave,
                    'O' => Self::Monster,
                    _ => return None,
                })
            }

            fn all_chars() -> Vec<char> {
                vec!['.', '#', 'O']
            }
        }

        let sea_monster = "\
..................#.
#....##....##....###
.#..#..#..#..#..#...";
        let sea_monster_pixels = TileSet::from_string(sea_monster, '#');
        let (sea_x_range, sea_y_range) = sea_monster_pixels.get_range().unwrap();
        let sea_x_width = sea_x_range.1 - sea_x_range.0;
        let sea_y_width = sea_y_range.1 - sea_y_range.0;

        // Check each orientation; only one should show sea monsters
        for &flipped in &[false, true] {
            for rotation in 0.. {
                let orientation = TileOrientation { rotation, flipped };
                let input = transform(&self.combined_pixels, orientation);
                let mut pixels_highlighted =
                    TileMap::new().with_tiles(input.iter().map(|p| (p, PixelTile::Wave)));
                let (x_range, y_range) = input.get_range().unwrap();
                for y in y_range.0..=y_range.1 - sea_y_width {
                    for x in x_range.0..=x_range.1 - sea_x_width {
                        let mut found_monster = true;
                        for sea_p in sea_monster_pixels.iter() {
                            let offset_p = Point { x, y } + sea_p;
                            if input.contains(&offset_p) == false {
                                found_monster = false;
                                break;
                            }
                        }

                        if found_monster == true {
                            for sea_p in sea_monster_pixels.iter() {
                                let offset_p = Point { x, y } + sea_p;
                                pixels_highlighted.insert(offset_p, PixelTile::Monster);
                            }
                        }
                    }
                }

                if pixels_highlighted
                    .values()
                    .filter(|c| c == &&PixelTile::Monster)
                    .count()
                    != 0
                {
                    // Found the answer!
                    //println!("{}", pixels_highlighted);
                    return pixels_highlighted
                        .values()
                        .filter(|c| c == &&PixelTile::Wave)
                        .count();
                }
            }
        }
        panic!("No sea monsters found!");
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.combined_pixels)
    }
}

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Vec<ImageTile> {
    many1(ImageTile::parser)(input).unwrap().1
}

#[aoc(day20, part1)]
pub fn part1(input: &[ImageTile]) -> u64 {
    let image = Image::from_image_tiles(input);
    let corners = image.find_corners();
    let product = corners.iter().product();
    assert_eq!(product, 111936085519519);
    product
}

#[aoc(day20, part2)]
pub fn part2(input: &[ImageTile]) -> usize {
    let mut image = Image::from_image_tiles(input);
    image.assemble();
    let roughness = image.find_sea_monsters();
    assert_eq!(roughness, 1792);
    roughness
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

    #[test]
    fn test_bit_reverse() {
        assert_eq!(bit_reverse(&0b0011010010, 10), 0b0100101100);
        assert_eq!(bit_reverse(&0b0001011001, 10), 0b1001101000);
        assert_eq!(bit_reverse(&0b0011100111, 10), 0b1110011100);
        assert_eq!(bit_reverse(&0b0111110010, 10), 0b0100111110);
    }

    #[test]
    fn test_image_tiles_sides() {
        let image_tile = ImageTile::parser(EXAMPLE_INPUT).unwrap().1;
        assert_eq!(
            image_tile.side_a_ids,
            [0b0011010010, 0b0001011001, 0b0011100111, 0b0111110010]
        );
        assert_eq!(
            image_tile.side_b_ids,
            [0b0100101100, 0b0111110010, 0b1110011100, 0b0001011001]
        );
    }

    #[test]
    fn test_get_side_id() {
        let image_tile = ImageTile::parser(EXAMPLE_INPUT).unwrap().1;
        let expected = [
            ((TileSide::Top, 0, false), 0b0011010010),
            ((TileSide::Top, 1, false), 0b0100111110),
            ((TileSide::Top, 2, false), 0b1110011100),
            ((TileSide::Top, 3, false), 0b0001011001),
            ((TileSide::Right, 0, false), 0b0001011001),
            ((TileSide::Right, 1, false), 0b0011010010),
            ((TileSide::Right, 2, false), 0b0100111110),
            ((TileSide::Right, 3, false), 0b1110011100),
            ((TileSide::Bottom, 0, false), 0b0011100111),
            ((TileSide::Bottom, 1, false), 0b1001101000),
            ((TileSide::Bottom, 2, false), 0b0100101100),
            ((TileSide::Bottom, 3, false), 0b0111110010),
            ((TileSide::Left, 0, false), 0b0111110010),
            ((TileSide::Left, 1, false), 0b0011100111),
            ((TileSide::Left, 2, false), 0b1001101000),
            ((TileSide::Left, 3, false), 0b0100101100),
            ((TileSide::Top, 0, true), 0b0100101100),
            ((TileSide::Top, 1, true), 0b1001101000),
            ((TileSide::Top, 2, true), 0b0011100111),
            ((TileSide::Top, 3, true), 0b0111110010),
            ((TileSide::Right, 0, true), 0b0111110010),
            ((TileSide::Right, 1, true), 0b0100101100),
            ((TileSide::Right, 2, true), 0b1001101000),
            ((TileSide::Right, 3, true), 0b0011100111),
            ((TileSide::Bottom, 0, true), 0b1110011100),
            ((TileSide::Bottom, 1, true), 0b0100111110),
            ((TileSide::Bottom, 2, true), 0b0011010010),
            ((TileSide::Bottom, 3, true), 0b0001011001),
            ((TileSide::Left, 0, true), 0b0001011001),
            ((TileSide::Left, 1, true), 0b1110011100),
            ((TileSide::Left, 2, true), 0b0100111110),
            ((TileSide::Left, 3, true), 0b0011010010),
        ];
        for &((s, r, f), ans) in expected.iter() {
            let o = TileOrientation {
                rotation: r,
                flipped: f,
            };
            assert_eq!(image_tile.get_side_id(s, o), ans);
        }
    }

    #[test]
    fn test_corner_product() {
        let image_tiles = input_generator(EXAMPLE_INPUT);
        let image = Image::from_image_tiles(&image_tiles);
        let corners = image.find_corners();
        let product: u64 = corners.iter().product();
        assert_eq!(product, 20899048083289);
    }

    #[test]
    fn test_transform() {
        let tiles: Vec<Point> = vec![(-2, -1), (2, -1), (2, 3), (-2, 3)]
            .into_iter()
            .map(|p| p.into())
            .collect();
        let expected = TileSet::new().with_tiles(&tiles);

        let transformed = expected.clone();
        let orientation = TileOrientation {
            rotation: 1,
            flipped: false,
        };
        let transformed = transform(&transformed, orientation);
        let transformed = transform(&transformed, orientation);
        let transformed = transform(&transformed, orientation);
        let transformed = transform(&transformed, orientation);
        assert_eq!(expected, transformed);

        let transformed = expected.clone();
        let orientation = TileOrientation {
            rotation: 2,
            flipped: false,
        };
        let transformed = transform(&transformed, orientation);
        let transformed = transform(&transformed, orientation);
        assert_eq!(expected, transformed);

        let transformed = expected.clone();
        let orientation = TileOrientation {
            rotation: 0,
            flipped: true,
        };
        let transformed = transform(&transformed, orientation);
        let transformed = transform(&transformed, orientation);
        assert_eq!(expected, transformed);
    }

    #[test]
    fn test_image_assemble() {
        let expected = "\
.#.#..#.##...#.##..#####
###....#.#....#..#......
##.##.###.#.#..######...
###.#####...#.#####.#..#
##.#....#.##.####...#.##
...########.#....#####.#
....#..#...##..#.#.###..
.####...#..#.....#......
#..#.##..#..###.#.##....
#.####..#.####.#.#.###..
###.#.#...#.######.#..##
#.####....##..########.#
##..##.#...#...#.#.#.#..
...#..#..#.#.##..###.###
.#.#....#.##.#...###.##.
###.#...#..#.##.######..
.#.#.###.##.##.#..#.##..
.####.###.#...###.#..#.#
..#.#..#..#.#.#.####.###
#..####...#.#.#.###.###.
#####..#####...###....##
#.##..#..#...#..####...#
.#.###..##..##..####.##.
...###...##...#...#..###";

        let image_tiles = input_generator(EXAMPLE_INPUT);
        let mut image = Image::from_image_tiles(&image_tiles);
        image.assemble();

        let original = image.combined_pixels.clone();
        let mut any = false;
        for &flipped in &[false, true] {
            for rotation in 0..MAX_ROT {
                let orientation = TileOrientation { rotation, flipped };
                image.combined_pixels = transform(&original, orientation);
                if image.to_string().trim() == expected {
                    any = true;
                }
            }
        }
        assert!(any);
    }

    #[test]
    fn test_find_sea_monsters() {
        let image_tiles = input_generator(EXAMPLE_INPUT);
        let mut image = Image::from_image_tiles(&image_tiles);
        image.assemble();
        let roughness = image.find_sea_monsters();
        assert_eq!(roughness, 273);
    }
}
