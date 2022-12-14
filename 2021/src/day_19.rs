/*
    --- Day 19: Beacon Scanner ---
    As your probe drifted down through this area, it released an assortment of beacons and scanners into the water. It's difficult to navigate in the pitch black open waters of the ocean trench, but if you can build a map of the trench using data from the scanners, you should be able to safely reach the bottom.

    The beacons and scanners float motionless in the water; they're designed to maintain the same position for long periods of time. Each scanner is capable of detecting all beacons in a large cube centered on the scanner; beacons that are at most 1000 units away from the scanner in each of the three axes (x, y, and z) have their precise position determined relative to the scanner. However, scanners cannot detect other scanners. The submarine has automatically summarized the relative positions of beacons detected by each scanner (your puzzle input).

    For example, if a scanner is at x,y,z coordinates 500,0,-500 and there are beacons at -500,1000,-1500 and 1501,0,-500, the scanner could report that the first beacon is at -1000,1000,-1000 (relative to the scanner) but would not detect the second beacon at all.

    Unfortunately, while each scanner can report the positions of all detected beacons relative to itself, the scanners do not know their own position. You'll need to determine the positions of the beacons and scanners yourself.

    The scanners and beacons map a single contiguous 3d region. This region can be reconstructed by finding pairs of scanners that have overlapping detection regions such that there are at least 12 beacons that both scanners detect within the overlap. By establishing 12 common beacons, you can precisely determine where the scanners are relative to each other, allowing you to reconstruct the beacon map one scanner at a time.

    For a moment, consider only two dimensions. Suppose you have the following scanner reports:

    --- scanner 0 ---
    0,2
    4,1
    3,3

    --- scanner 1 ---
    -1,-1
    -5,0
    -2,1
    Drawing x increasing rightward, y increasing upward, scanners as S, and beacons as B, scanner 0 detects this:

    ...B.
    B....
    ....B
    S....
    Scanner 1 detects this:

    ...B..
    B....S
    ....B.
    For this example, assume scanners only need 3 overlapping beacons. Then, the beacons visible to both scanners overlap to produce the following complete map:

    ...B..
    B....S
    ....B.
    S.....
    Unfortunately, there's a second problem: the scanners also don't know their rotation or facing direction. Due to magnetic alignment, each scanner is rotated some integer number of 90-degree turns around all of the x, y, and z axes. That is, one scanner might call a direction positive x, while another scanner might call that direction negative y. Or, two scanners might agree on which direction is positive x, but one scanner might be upside-down from the perspective of the other scanner. In total, each scanner could be in any of 24 different orientations: facing positive or negative x, y, or z, and considering any of four directions "up" from that facing.

    For example, here is an arrangement of beacons as seen from a scanner in the same position but in different orientations:

    --- scanner 0 ---
    -1,-1,1
    -2,-2,2
    -3,-3,3
    -2,-3,1
    5,6,-4
    8,0,7

    --- scanner 0 ---
    1,-1,1
    2,-2,2
    3,-3,3
    2,-1,3
    -5,4,-6
    -8,-7,0

    --- scanner 0 ---
    -1,-1,-1
    -2,-2,-2
    -3,-3,-3
    -1,-3,-2
    4,6,5
    -7,0,8

    --- scanner 0 ---
    1,1,-1
    2,2,-2
    3,3,-3
    1,3,-2
    -4,-6,5
    7,0,8

    --- scanner 0 ---
    1,1,1
    2,2,2
    3,3,3
    3,1,2
    -6,-4,-5
    0,7,-8
    By finding pairs of scanners that both see at least 12 of the same beacons, you can assemble the entire map. For example, consider the following report:

    --- scanner 0 ---
    404,-588,-901
    528,-643,409
    -838,591,734
    390,-675,-793
    -537,-823,-458
    -485,-357,347
    -345,-311,381
    -661,-816,-575
    -876,649,763
    -618,-824,-621
    553,345,-567
    474,580,667
    -447,-329,318
    -584,868,-557
    544,-627,-890
    564,392,-477
    455,729,728
    -892,524,684
    -689,845,-530
    423,-701,434
    7,-33,-71
    630,319,-379
    443,580,662
    -789,900,-551
    459,-707,401

    --- scanner 1 ---
    686,422,578
    605,423,415
    515,917,-361
    -336,658,858
    95,138,22
    -476,619,847
    -340,-569,-846
    567,-361,727
    -460,603,-452
    669,-402,600
    729,430,532
    -500,-761,534
    -322,571,750
    -466,-666,-811
    -429,-592,574
    -355,545,-477
    703,-491,-529
    -328,-685,520
    413,935,-424
    -391,539,-444
    586,-435,557
    -364,-763,-893
    807,-499,-711
    755,-354,-619
    553,889,-390

    --- scanner 2 ---
    649,640,665
    682,-795,504
    -784,533,-524
    -644,584,-595
    -588,-843,648
    -30,6,44
    -674,560,763
    500,723,-460
    609,671,-379
    -555,-800,653
    -675,-892,-343
    697,-426,-610
    578,704,681
    493,664,-388
    -671,-858,530
    -667,343,800
    571,-461,-707
    -138,-166,112
    -889,563,-600
    646,-828,498
    640,759,510
    -630,509,768
    -681,-892,-333
    673,-379,-804
    -742,-814,-386
    577,-820,562

    --- scanner 3 ---
    -589,542,597
    605,-692,669
    -500,565,-823
    -660,373,557
    -458,-679,-417
    -488,449,543
    -626,468,-788
    338,-750,-386
    528,-832,-391
    562,-778,733
    -938,-730,414
    543,643,-506
    -524,371,-870
    407,773,750
    -104,29,83
    378,-903,-323
    -778,-728,485
    426,699,580
    -438,-605,-362
    -469,-447,-387
    509,732,623
    647,635,-688
    -868,-804,481
    614,-800,639
    595,780,-596

    --- scanner 4 ---
    727,592,562
    -293,-554,779
    441,611,-461
    -714,465,-776
    -743,427,-804
    -660,-479,-426
    832,-632,460
    927,-485,-438
    408,393,-506
    466,436,-512
    110,16,151
    -258,-428,682
    -393,719,612
    -211,-452,876
    808,-476,-593
    -575,615,604
    -485,667,467
    -680,325,-822
    -627,-443,-432
    872,-547,-609
    833,512,582
    807,604,487
    839,-516,451
    891,-625,532
    -652,-548,-490
    30,-46,-14
    Because all coordinates are relative, in this example, all "absolute" positions will be expressed relative to scanner 0 (using the orientation of scanner 0 and as if scanner 0 is at coordinates 0,0,0).

    Scanners 0 and 1 have overlapping detection cubes; the 12 beacons they both detect (relative to scanner 0) are at the following coordinates:

    -618,-824,-621
    -537,-823,-458
    -447,-329,318
    404,-588,-901
    544,-627,-890
    528,-643,409
    -661,-816,-575
    390,-675,-793
    423,-701,434
    -345,-311,381
    459,-707,401
    -485,-357,347
    These same 12 beacons (in the same order) but from the perspective of scanner 1 are:

    686,422,578
    605,423,415
    515,917,-361
    -336,658,858
    -476,619,847
    -460,603,-452
    729,430,532
    -322,571,750
    -355,545,-477
    413,935,-424
    -391,539,-444
    553,889,-390
    Because of this, scanner 1 must be at 68,-1246,-43 (relative to scanner 0).

    Scanner 4 overlaps with scanner 1; the 12 beacons they both detect (relative to scanner 0) are:

    459,-707,401
    -739,-1745,668
    -485,-357,347
    432,-2009,850
    528,-643,409
    423,-701,434
    -345,-311,381
    408,-1815,803
    534,-1912,768
    -687,-1600,576
    -447,-329,318
    -635,-1737,486
    So, scanner 4 is at -20,-1133,1061 (relative to scanner 0).

    Following this process, scanner 2 must be at 1105,-1205,1229 (relative to scanner 0) and scanner 3 must be at -92,-2380,-20 (relative to scanner 0).

    The full list of beacons (relative to scanner 0) is:

    -892,524,684
    -876,649,763
    -838,591,734
    -789,900,-551
    -739,-1745,668
    -706,-3180,-659
    -697,-3072,-689
    -689,845,-530
    -687,-1600,576
    -661,-816,-575
    -654,-3158,-753
    -635,-1737,486
    -631,-672,1502
    -624,-1620,1868
    -620,-3212,371
    -618,-824,-621
    -612,-1695,1788
    -601,-1648,-643
    -584,868,-557
    -537,-823,-458
    -532,-1715,1894
    -518,-1681,-600
    -499,-1607,-770
    -485,-357,347
    -470,-3283,303
    -456,-621,1527
    -447,-329,318
    -430,-3130,366
    -413,-627,1469
    -345,-311,381
    -36,-1284,1171
    -27,-1108,-65
    7,-33,-71
    12,-2351,-103
    26,-1119,1091
    346,-2985,342
    366,-3059,397
    377,-2827,367
    390,-675,-793
    396,-1931,-563
    404,-588,-901
    408,-1815,803
    423,-701,434
    432,-2009,850
    443,580,662
    455,729,728
    456,-540,1869
    459,-707,401
    465,-695,1988
    474,580,667
    496,-1584,1900
    497,-1838,-617
    527,-524,1933
    528,-643,409
    534,-1912,768
    544,-627,-890
    553,345,-567
    564,392,-477
    568,-2007,-577
    605,-1665,1952
    612,-1593,1893
    630,319,-379
    686,-3108,-505
    776,-3184,-501
    846,-3110,-434
    1135,-1161,1235
    1243,-1093,1063
    1660,-552,429
    1693,-557,386
    1735,-437,1738
    1749,-1800,1813
    1772,-405,1572
    1776,-675,371
    1779,-442,1789
    1780,-1548,337
    1786,-1538,337
    1847,-1591,415
    1889,-1729,1762
    1994,-1805,1792
    In total, there are 79 beacons.

    Assemble the full map of beacons. How many beacons are there?

    --- Part Two ---
    Sometimes, it's a good idea to appreciate just how big the ocean is. Using the Manhattan distance, how far apart do the scanners get?

    In the above example, scanners 2 (1105,-1205,1229) and 3 (-92,-2380,-20) are the largest Manhattan distance apart. In total, they are 1197 + 1175 + 1249 = 3621 units apart.

    What is the largest Manhattan distance between any two scanners?
*/

use common::{unsigned, Point3};
use nom::{
    bytes::complete::tag,
    character::complete::multispace0,
    multi::many1,
    sequence::{preceded, tuple},
    IResult,
};
use std::{cmp::Ordering, collections::HashSet};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Xyz {
    X,
    Y,
    Z,
    NX,
    NY,
    NZ,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Orientation {
    facing: Xyz,
    n: u32,
}

impl Orientation {
    fn new() -> Self {
        Self {
            facing: Xyz::NZ,
            n: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Scanner {
    id: u32,
    beacons: Vec<Point3>,
    position: Option<Point3>,
    orientation: Option<Orientation>,
    distances: HashSet<u32>,
}

impl Scanner {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (_, _, id, _, mut beacons)) = tuple((
            multispace0,
            tag("--- scanner "),
            unsigned,
            tag(" ---"),
            many1(preceded(multispace0, Point3::parser)),
        ))(input)?;

        beacons.sort_unstable();

        Ok((
            input,
            Self {
                beacons,
                id,
                position: None,
                orientation: None,
                distances: HashSet::new(),
            },
        ))
    }

    fn gen_distances(&mut self) {
        let mut distances = HashSet::new();
        for a in &self.beacons {
            for b in &self.beacons {
                let dist = Point3::manhattan(*a, *b);
                distances.insert(dist);
            }
        }
        self.distances = distances;
    }

    fn orient(&self, orientation: Orientation) -> Self {
        let mut beacons = Vec::new();
        for p in &self.beacons {
            let new_p = match orientation.facing {
                Xyz::X => rotate_point_around_axis(p, Xyz::Y, 3),
                Xyz::Y => rotate_point_around_axis(p, Xyz::X, 1),
                Xyz::Z => rotate_point_around_axis(p, Xyz::Y, 2),
                Xyz::NX => rotate_point_around_axis(p, Xyz::Y, 1),
                Xyz::NY => rotate_point_around_axis(p, Xyz::X, 3),
                Xyz::NZ => *p,
            };
            let new_p = rotate_point_around_axis(&new_p, orientation.facing, orientation.n);
            beacons.push(new_p);
        }

        beacons.sort_unstable();

        Self {
            id: self.id,
            beacons,
            position: self.position,
            orientation: Some(orientation),
            distances: self.distances.clone(),
        }
    }

    fn check_overlap(&self, other: &Self, overlap_criteria: u32) -> Option<Point3> {
        for self_p in &self.beacons {
            for other_p in &other.beacons {
                // Use self - other here because it makes the final answer more intuitive e.g. the answer has positive x if other is to the right of self.
                let offset = self_p - other_p;
                let mut count = 0;
                for (i, check_p) in other.beacons.iter().enumerate() {
                    if (other.beacons.len() - i) < (overlap_criteria - count) as usize {
                        // Not enough beacons left to possibly reach the criteria count
                        break;
                    }

                    let offset_check = check_p + offset;
                    if self.beacons.contains(&offset_check) {
                        count += 1;
                    }
                }

                match count.cmp(&overlap_criteria) {
                    Ordering::Equal => return Some(offset),
                    Ordering::Greater => panic!("They overlap suspiciously well"),
                    Ordering::Less => (),
                }
            }
        }

        None
    }

    fn check_overlap_oriented(&self, other: &Self, overlap_criteria: u32) -> Option<Self> {
        for facing in [Xyz::X, Xyz::Y, Xyz::Z, Xyz::NX, Xyz::NY, Xyz::NZ] {
            for n in 0..4 {
                let oriented = other.orient(Orientation { facing, n });
                if let Some(s) = self.check_overlap(&oriented, overlap_criteria) {
                    return Some(Self {
                        position: self.position.map(|x| x + s),
                        ..oriented
                    });
                }
            }
        }
        None
    }

    fn check_overlap_by_distance(&self, other: &Self, overlap_criteria: u32) -> bool {
        self.distances.intersection(&other.distances).count() >= overlap_criteria as usize
    }
}

fn rotate_point_around_axis(p: &Point3, axis: Xyz, n: u32) -> Point3 {
    assert!(n < 4);
    let mut new_p = *p;
    for _ in 0..n {
        new_p = match axis {
            Xyz::X => Point3 {
                x: new_p.x,
                y: -new_p.z,
                z: new_p.y,
            },
            Xyz::Y => Point3 {
                x: new_p.z,
                y: new_p.y,
                z: -new_p.x,
            },
            Xyz::Z => Point3 {
                x: -new_p.y,
                y: new_p.x,
                z: new_p.z,
            },
            Xyz::NX => Point3 {
                x: new_p.x,
                y: new_p.z,
                z: -new_p.y,
            },
            Xyz::NY => Point3 {
                x: -new_p.z,
                y: new_p.y,
                z: new_p.x,
            },
            Xyz::NZ => Point3 {
                x: new_p.y,
                y: -new_p.x,
                z: new_p.z,
            },
        }
    }
    new_p
}

fn find_all_positions(scanners: &[Scanner]) -> Vec<Scanner> {
    let mut undetermined = scanners.to_vec();
    for scanner in &mut undetermined {
        scanner.gen_distances();
    }

    let mut origin = undetermined.remove(0);
    origin.position = Some(Point3::origin());
    origin.orientation = Some(Orientation::new());
    let mut oriented = vec![origin];

    let mut tried: HashSet<(u32, u32)> = HashSet::new();

    let mut keep: Vec<bool> = Vec::new();
    loop {
        keep.clear();
        'undet: for undet in &undetermined {
            for i in 0..oriented.len() {
                if tried.contains(&(oriented[i].id, undet.id)) == true {
                    // We already tried it, skip it
                    continue;
                }

                // 67 because if 12 points overlap there are 66 (12 choose 2) distances between them.
                // Plus one because we didn't remove the identity distance.
                if oriented[i].check_overlap_by_distance(undet, 67) == true {
                    if let Some(s) = oriented[i].check_overlap_oriented(undet, 12) {
                        oriented.push(s);
                        keep.push(false);
                        continue 'undet;
                    }
                }
                // Mark that we tried this combination so we don't do it again
                tried.insert((oriented[i].id, undet.id));
            }
            keep.push(true);
        }

        let mut keep_iter = keep.iter();
        undetermined.retain(|_| *keep_iter.next().unwrap());

        if undetermined.is_empty() == true {
            break;
        }
    }

    oriented
}

fn unique_beacons(scanners: &[Scanner]) -> Vec<Point3> {
    let oriented = find_all_positions(scanners);
    let mut total_beacons: Vec<Point3> = Vec::new();
    for s in &oriented {
        total_beacons.extend(s.beacons.iter().map(|p| p + s.position.unwrap()));
    }
    total_beacons.sort_unstable_by(Point3::cmp_xyz);
    total_beacons.dedup();
    total_beacons
}

fn largest_scanner_distance(scanners: &[Scanner]) -> u32 {
    let mut largest = 0;

    for a in scanners {
        for b in scanners {
            let dist = Point3::manhattan(a.position.unwrap(), b.position.unwrap());
            if dist > largest {
                largest = dist;
            }
        }
    }

    largest
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Vec<Scanner> {
    many1(Scanner::parser)(input).unwrap().1
}

#[aoc(day19, part1)]
pub fn part1(input: &[Scanner]) -> usize {
    let beacons = unique_beacons(input);
    let num_beacons = beacons.len();
    assert_eq!(num_beacons, 467);
    num_beacons
}

#[aoc(day19, part2)]
pub fn part2(input: &[Scanner]) -> u32 {
    let oriented = find_all_positions(input);
    let distance = largest_scanner_distance(&oriented);
    assert_eq!(distance, 12226);
    distance
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    static EXAMPLE_INPUT1: &str = "\
--- scanner 0 ---
0,2,0
4,1,0
3,3,0

--- scanner 1 ---
-1,-1,0
-5,0,0
-2,1,0";

    static EXAMPLE_INPUT2: &str = "\
--- scanner 0 ---
-1,-1,1
-2,-2,2
-3,-3,3
-2,-3,1
5,6,-4
8,0,7

--- scanner 0 ---
1,-1,1
2,-2,2
3,-3,3
2,-1,3
-5,4,-6
-8,-7,0

--- scanner 0 ---
-1,-1,-1
-2,-2,-2
-3,-3,-3
-1,-3,-2
4,6,5
-7,0,8

--- scanner 0 ---
1,1,-1
2,2,-2
3,3,-3
1,3,-2
-4,-6,5
7,0,8

--- scanner 0 ---
1,1,1
2,2,2
3,3,3
3,1,2
-6,-4,-5
0,7,-8";

    static EXAMPLE_INPUT3: &str = "\
--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14
";

    #[test]
    fn test_orient() {
        let scanners = input_generator(EXAMPLE_INPUT2);

        let orientation = Orientation {
            facing: Xyz::NZ,
            n: 0,
        };
        assert_eq!(scanners[0].orient(orientation).beacons, scanners[0].beacons);

        let orientation = Orientation {
            facing: Xyz::Y,
            n: 2,
        };
        assert_eq!(scanners[0].orient(orientation).beacons, scanners[1].beacons);

        let orientation = Orientation {
            facing: Xyz::X,
            n: 0,
        };
        assert_eq!(scanners[0].orient(orientation).beacons, scanners[2].beacons);

        let orientation = Orientation {
            facing: Xyz::NX,
            n: 2,
        };
        assert_eq!(scanners[0].orient(orientation).beacons, scanners[3].beacons);

        let orientation = Orientation {
            facing: Xyz::NY,
            n: 3,
        };
        assert_eq!(scanners[0].orient(orientation).beacons, scanners[4].beacons);
    }

    #[test]
    fn test_check_overlap() {
        let scanners = input_generator(EXAMPLE_INPUT1);
        let res = scanners[0].check_overlap(&scanners[1], 3);
        assert_eq!(res, Some((5, 2, 0).into()));
    }

    #[test]
    fn test_check_overlap_oriented() {
        let scanners = input_generator(EXAMPLE_INPUT3);

        let mut oriented: Vec<Scanner> = scanners.clone();
        oriented[0].position = Some(Point3::origin());
        oriented[0].orientation = Some(Orientation::new());

        oriented[1] = oriented[0]
            .check_overlap_oriented(&scanners[1], 12)
            .unwrap();
        assert_eq!(oriented[1].position, Some((68, -1246, -43).into()));

        oriented[4] = oriented[1]
            .check_overlap_oriented(&scanners[4], 12)
            .unwrap();
        assert_eq!(oriented[4].position, Some((-20, -1133, 1061).into()));

        oriented[2] = oriented[4]
            .check_overlap_oriented(&scanners[2], 12)
            .unwrap();
        assert_eq!(oriented[2].position, Some((1105, -1205, 1229).into()));

        oriented[3] = oriented[1]
            .check_overlap_oriented(&scanners[3], 12)
            .unwrap();
        assert_eq!(oriented[3].position, Some((-92, -2380, -20).into()));
    }

    #[test]
    fn test_find_all_positions() {
        let scanners = input_generator(EXAMPLE_INPUT3);

        let oriented = find_all_positions(&scanners);
        let positions: HashSet<Point3> = oriented.into_iter().filter_map(|s| s.position).collect();
        assert_eq!(
            positions,
            vec![
                (0, 0, 0).into(),
                (68, -1246, -43).into(),
                (1105, -1205, 1229).into(),
                (-92, -2380, -20).into(),
                (-20, -1133, 1061).into(),
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn test_unique_beacons() {
        let scanners = input_generator(EXAMPLE_INPUT3);
        let beacons = unique_beacons(&scanners);
        assert_eq!(beacons.len(), 79);
        assert_eq!(
            beacons,
            vec![
                (-892, 524, 684).into(),
                (-876, 649, 763).into(),
                (-838, 591, 734).into(),
                (-789, 900, -551).into(),
                (-739, -1745, 668).into(),
                (-706, -3180, -659).into(),
                (-697, -3072, -689).into(),
                (-689, 845, -530).into(),
                (-687, -1600, 576).into(),
                (-661, -816, -575).into(),
                (-654, -3158, -753).into(),
                (-635, -1737, 486).into(),
                (-631, -672, 1502).into(),
                (-624, -1620, 1868).into(),
                (-620, -3212, 371).into(),
                (-618, -824, -621).into(),
                (-612, -1695, 1788).into(),
                (-601, -1648, -643).into(),
                (-584, 868, -557).into(),
                (-537, -823, -458).into(),
                (-532, -1715, 1894).into(),
                (-518, -1681, -600).into(),
                (-499, -1607, -770).into(),
                (-485, -357, 347).into(),
                (-470, -3283, 303).into(),
                (-456, -621, 1527).into(),
                (-447, -329, 318).into(),
                (-430, -3130, 366).into(),
                (-413, -627, 1469).into(),
                (-345, -311, 381).into(),
                (-36, -1284, 1171).into(),
                (-27, -1108, -65).into(),
                (7, -33, -71).into(),
                (12, -2351, -103).into(),
                (26, -1119, 1091).into(),
                (346, -2985, 342).into(),
                (366, -3059, 397).into(),
                (377, -2827, 367).into(),
                (390, -675, -793).into(),
                (396, -1931, -563).into(),
                (404, -588, -901).into(),
                (408, -1815, 803).into(),
                (423, -701, 434).into(),
                (432, -2009, 850).into(),
                (443, 580, 662).into(),
                (455, 729, 728).into(),
                (456, -540, 1869).into(),
                (459, -707, 401).into(),
                (465, -695, 1988).into(),
                (474, 580, 667).into(),
                (496, -1584, 1900).into(),
                (497, -1838, -617).into(),
                (527, -524, 1933).into(),
                (528, -643, 409).into(),
                (534, -1912, 768).into(),
                (544, -627, -890).into(),
                (553, 345, -567).into(),
                (564, 392, -477).into(),
                (568, -2007, -577).into(),
                (605, -1665, 1952).into(),
                (612, -1593, 1893).into(),
                (630, 319, -379).into(),
                (686, -3108, -505).into(),
                (776, -3184, -501).into(),
                (846, -3110, -434).into(),
                (1135, -1161, 1235).into(),
                (1243, -1093, 1063).into(),
                (1660, -552, 429).into(),
                (1693, -557, 386).into(),
                (1735, -437, 1738).into(),
                (1749, -1800, 1813).into(),
                (1772, -405, 1572).into(),
                (1776, -675, 371).into(),
                (1779, -442, 1789).into(),
                (1780, -1548, 337).into(),
                (1786, -1538, 337).into(),
                (1847, -1591, 415).into(),
                (1889, -1729, 1762).into(),
                (1994, -1805, 1792).into(),
            ]
        );
    }

    #[test]
    fn test_largest_scanner_distance() {
        let scanners = input_generator(EXAMPLE_INPUT3);
        let oriented = find_all_positions(&scanners);
        let distance = largest_scanner_distance(&oriented);
        assert_eq!(distance, 3621);
    }
}
