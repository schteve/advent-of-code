/*
    --- Day 22: Reactor Reboot ---
    Operating at these extreme ocean depths has overloaded the submarine's reactor; it needs to be rebooted.

    The reactor core is made up of a large 3-dimensional grid made up entirely of cubes, one cube per integer 3-dimensional coordinate (x,y,z). Each cube can be either on or off; at the start of the reboot process, they are all off. (Could it be an old model of a reactor you've seen before?)

    To reboot the reactor, you just need to set all of the cubes to either on or off by following a list of reboot steps (your puzzle input). Each step specifies a cuboid (the set of all cubes that have coordinates which fall within ranges for x, y, and z) and whether to turn all of the cubes in that cuboid on or off.

    For example, given these reboot steps:

    on x=10..12,y=10..12,z=10..12
    on x=11..13,y=11..13,z=11..13
    off x=9..11,y=9..11,z=9..11
    on x=10..10,y=10..10,z=10..10
    The first step (on x=10..12,y=10..12,z=10..12) turns on a 3x3x3 cuboid consisting of 27 cubes:

    10,10,10
    10,10,11
    10,10,12
    10,11,10
    10,11,11
    10,11,12
    10,12,10
    10,12,11
    10,12,12
    11,10,10
    11,10,11
    11,10,12
    11,11,10
    11,11,11
    11,11,12
    11,12,10
    11,12,11
    11,12,12
    12,10,10
    12,10,11
    12,10,12
    12,11,10
    12,11,11
    12,11,12
    12,12,10
    12,12,11
    12,12,12
    The second step (on x=11..13,y=11..13,z=11..13) turns on a 3x3x3 cuboid that overlaps with the first. As a result, only 19 additional cubes turn on; the rest are already on from the previous step:

    11,11,13
    11,12,13
    11,13,11
    11,13,12
    11,13,13
    12,11,13
    12,12,13
    12,13,11
    12,13,12
    12,13,13
    13,11,11
    13,11,12
    13,11,13
    13,12,11
    13,12,12
    13,12,13
    13,13,11
    13,13,12
    13,13,13
    The third step (off x=9..11,y=9..11,z=9..11) turns off a 3x3x3 cuboid that overlaps partially with some cubes that are on, ultimately turning off 8 cubes:

    10,10,10
    10,10,11
    10,11,10
    10,11,11
    11,10,10
    11,10,11
    11,11,10
    11,11,11
    The final step (on x=10..10,y=10..10,z=10..10) turns on a single cube, 10,10,10. After this last step, 39 cubes are on.

    The initialization procedure only uses cubes that have x, y, and z positions of at least -50 and at most 50. For now, ignore cubes outside this region.

    Here is a larger example:

    on x=-20..26,y=-36..17,z=-47..7
    on x=-20..33,y=-21..23,z=-26..28
    on x=-22..28,y=-29..23,z=-38..16
    on x=-46..7,y=-6..46,z=-50..-1
    on x=-49..1,y=-3..46,z=-24..28
    on x=2..47,y=-22..22,z=-23..27
    on x=-27..23,y=-28..26,z=-21..29
    on x=-39..5,y=-6..47,z=-3..44
    on x=-30..21,y=-8..43,z=-13..34
    on x=-22..26,y=-27..20,z=-29..19
    off x=-48..-32,y=26..41,z=-47..-37
    on x=-12..35,y=6..50,z=-50..-2
    off x=-48..-32,y=-32..-16,z=-15..-5
    on x=-18..26,y=-33..15,z=-7..46
    off x=-40..-22,y=-38..-28,z=23..41
    on x=-16..35,y=-41..10,z=-47..6
    off x=-32..-23,y=11..30,z=-14..3
    on x=-49..-5,y=-3..45,z=-29..18
    off x=18..30,y=-20..-8,z=-3..13
    on x=-41..9,y=-7..43,z=-33..15
    on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
    on x=967..23432,y=45373..81175,z=27513..53682
    The last two steps are fully outside the initialization procedure area; all other steps are fully within it. After executing these steps in the initialization procedure region, 590784 cubes are on.

    Execute the reboot steps. Afterward, considering only cubes in the region x=-50..50,y=-50..50,z=-50..50, how many cubes are on?

    --- Part Two ---
    Now that the initialization procedure is complete, you can reboot the reactor.

    Starting with all cubes off, run all of the reboot steps for all cubes in the reactor.

    Consider the following reboot steps:

    on x=-5..47,y=-31..22,z=-19..33
    on x=-44..5,y=-27..21,z=-14..35
    on x=-49..-1,y=-11..42,z=-10..38
    on x=-20..34,y=-40..6,z=-44..1
    off x=26..39,y=40..50,z=-2..11
    on x=-41..5,y=-41..6,z=-36..8
    off x=-43..-33,y=-45..-28,z=7..25
    on x=-33..15,y=-32..19,z=-34..11
    off x=35..47,y=-46..-34,z=-11..5
    on x=-14..36,y=-6..44,z=-16..29
    on x=-57795..-6158,y=29564..72030,z=20435..90618
    on x=36731..105352,y=-21140..28532,z=16094..90401
    on x=30999..107136,y=-53464..15513,z=8553..71215
    on x=13528..83982,y=-99403..-27377,z=-24141..23996
    on x=-72682..-12347,y=18159..111354,z=7391..80950
    on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
    on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
    on x=-52752..22273,y=-49450..9096,z=54442..119054
    on x=-29982..40483,y=-108474..-28371,z=-24328..38471
    on x=-4958..62750,y=40422..118853,z=-7672..65583
    on x=55694..108686,y=-43367..46958,z=-26781..48729
    on x=-98497..-18186,y=-63569..3412,z=1232..88485
    on x=-726..56291,y=-62629..13224,z=18033..85226
    on x=-110886..-34664,y=-81338..-8658,z=8914..63723
    on x=-55829..24974,y=-16897..54165,z=-121762..-28058
    on x=-65152..-11147,y=22489..91432,z=-58782..1780
    on x=-120100..-32970,y=-46592..27473,z=-11695..61039
    on x=-18631..37533,y=-124565..-50804,z=-35667..28308
    on x=-57817..18248,y=49321..117703,z=5745..55881
    on x=14781..98692,y=-1341..70827,z=15753..70151
    on x=-34419..55919,y=-19626..40991,z=39015..114138
    on x=-60785..11593,y=-56135..2999,z=-95368..-26915
    on x=-32178..58085,y=17647..101866,z=-91405..-8878
    on x=-53655..12091,y=50097..105568,z=-75335..-4862
    on x=-111166..-40997,y=-71714..2688,z=5609..50954
    on x=-16602..70118,y=-98693..-44401,z=5197..76897
    on x=16383..101554,y=4615..83635,z=-44907..18747
    off x=-95822..-15171,y=-19987..48940,z=10804..104439
    on x=-89813..-14614,y=16069..88491,z=-3297..45228
    on x=41075..99376,y=-20427..49978,z=-52012..13762
    on x=-21330..50085,y=-17944..62733,z=-112280..-30197
    on x=-16478..35915,y=36008..118594,z=-7885..47086
    off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
    off x=2032..69770,y=-71013..4824,z=7471..94418
    on x=43670..120875,y=-42068..12382,z=-24787..38892
    off x=37514..111226,y=-45862..25743,z=-16714..54663
    off x=25699..97951,y=-30668..59918,z=-15349..69697
    off x=-44271..17935,y=-9516..60759,z=49131..112598
    on x=-61695..-5813,y=40978..94975,z=8655..80240
    off x=-101086..-9439,y=-7088..67543,z=33935..83858
    off x=18020..114017,y=-48931..32606,z=21474..89843
    off x=-77139..10506,y=-89994..-18797,z=-80..59318
    off x=8476..79288,y=-75520..11602,z=-96624..-24783
    on x=-47488..-1262,y=24338..100707,z=16292..72967
    off x=-84341..13987,y=2429..92914,z=-90671..-1318
    off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
    off x=-27365..46395,y=31009..98017,z=15428..76570
    off x=-70369..-16548,y=22648..78696,z=-1892..86821
    on x=-53470..21291,y=-120233..-33476,z=-44150..38147
    off x=-93533..-4276,y=-16170..68771,z=-104985..-24507
    After running the above reboot steps, 2758514936282235 cubes are on. (Just for fun, 474140 of those are also in the initialization procedure region.)

    Starting again with all cubes off, execute all reboot steps. Afterward, considering all cubes, how many cubes are on?
*/

use common::{signed, Point3, Range3};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char, multispace0},
    combinator::map,
    multi::many1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

fn range3_parser(input: &str) -> IResult<&str, Range3> {
    let (input, (_, x0, _, x1, _, y0, _, y1, _, z0, _, z1)) = tuple((
        tag("x="),
        signed,
        tag(".."),
        signed::<i32>,
        tag(",y="),
        signed,
        tag(".."),
        signed::<i32>,
        tag(",z="),
        signed,
        tag(".."),
        signed::<i32>,
    ))(input)?;

    Ok((
        input,
        Range3 {
            x: (x0, x1 + 1), // Make it exclusive
            y: (y0, y1 + 1),
            z: (z0, z1 + 1),
        },
    ))
}

#[derive(Clone)]
pub struct Cuboid {
    on_off: bool,
    range: Range3,
}

impl Cuboid {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (on_off, range)) = preceded(
            multispace0,
            separated_pair(
                map(alpha1, |x| match x {
                    "on" => true,
                    "off" => false,
                    _ => panic!("Invalid: \"{}\"", x),
                }),
                char(' '),
                range3_parser,
            ),
        )(input)?;

        Ok((input, Self { on_off, range }))
    }
}

fn intersect_split(a: Range3, b: Range3) -> Vec<Range3> {
    // Break 'a' into pieces, this is my last resort.
    // 'b' is not added here so that each existing region can be split without double-counting b's regions.

    // First define the control points. We divide the plane into 9 regions defined by the boundaries on each dimension.
    // Control points must be sorted - it doesn't matter which region they belong to because we test if they are 'on'.
    let mut cx = [a.x.0, a.x.1, b.x.0, b.x.1];
    cx.sort_unstable();
    let mut cy = [a.y.0, a.y.1, b.y.0, b.y.1];
    cy.sort_unstable();
    let mut cz = [a.z.0, a.z.1, b.z.0, b.z.1];
    cz.sort_unstable();

    // Then generate the regions based on the control points
    let mut regions = Vec::new();
    for x in cx.windows(2) {
        for y in cy.windows(2) {
            for z in cz.windows(2) {
                // If any of these windows is 0 length then skip this volume
                if x[0] == x[1] || y[0] == y[1] || z[0] == z[1] {
                    continue;
                }

                // We're only splitting 'a' so include region if it's in 'a' but not in 'b'
                let point = Point3 {
                    x: x[0],
                    y: y[0],
                    z: z[0],
                };
                if a.contains_exclusive(&point) == true && b.contains_exclusive(&point) == false {
                    let new_region = Range3 {
                        x: (x[0], x[1]),
                        y: (y[0], y[1]),
                        z: (z[0], z[1]),
                    };
                    regions.push(new_region);
                }
            }
        }
    }
    regions
}

fn ranges_overlap(a: &Range3, b: &Range3) -> bool {
    a.x.0 < b.x.1
        && b.x.0 < a.x.1
        && a.y.0 < b.y.1
        && b.y.0 < a.y.1
        && a.z.0 < b.z.1
        && b.z.0 < a.z.1
}

fn reboot(cuboids: &[Cuboid]) -> Vec<Range3> {
    let mut cuboid_iter = cuboids.iter();
    let first = cuboid_iter.next().unwrap(); // Assume the first cuboid is on
    assert!(first.on_off);
    let mut regions = vec![first.range];
    for cuboid in cuboid_iter {
        let mut new_regions = Vec::new();
        for region in regions {
            if ranges_overlap(&region, &cuboid.range) == true {
                let mut new = intersect_split(region, cuboid.range);
                new_regions.append(&mut new);
            } else {
                new_regions.push(region);
            }
        }
        if cuboid.on_off == true {
            new_regions.push(cuboid.range);
        }
        regions = new_regions;
    }
    regions
}

fn count_on(regions: &[Range3]) -> usize {
    regions.iter().map(|r| r.volume_exclusive() as usize).sum()
}

fn restrict_range_50(cuboids: &mut Vec<Cuboid>) {
    #[rustfmt::skip]
    let range_50 = [
        Cuboid { range: Range3 { x: (-1_000_000, -50), y: (-1_000_000, 1_000_000), z: (-1_000_000, 1_000_000) }, on_off: false },
        Cuboid { range: Range3 { x: (-1_000_000, 1_000_000), y: (-1_000_000, -50), z: (-1_000_000, 1_000_000) }, on_off: false },
        Cuboid { range: Range3 { x: (-1_000_000, 1_000_000), y: (-1_000_000, 1_000_000), z: (-1_000_000, -50) }, on_off: false },
        Cuboid { range: Range3 { x: (51, 1_000_000), y: (-1_000_000, 1_000_000), z: (-1_000_000, 1_000_000) }, on_off: false },
        Cuboid { range: Range3 { x: (-1_000_000, 1_000_000), y: (51, 1_000_000), z: (-1_000_000, 1_000_000) }, on_off: false },
        Cuboid { range: Range3 { x: (-1_000_000, 1_000_000), y: (-1_000_000, 1_000_000), z: (51, 1_000_000) }, on_off: false },
    ];
    cuboids.extend(range_50);
}

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Vec<Cuboid> {
    many1(Cuboid::parser)(input).unwrap().1
}

#[aoc(day22, part1)]
pub fn part1(input: &[Cuboid]) -> usize {
    let mut cuboids = input.to_vec();
    restrict_range_50(&mut cuboids);
    let grid = reboot(&cuboids);
    let count = count_on(&grid);
    assert_eq!(count, 587785);
    count
}

#[aoc(day22, part2)]
pub fn part2(input: &[Cuboid]) -> usize {
    let grid = reboot(input);
    let count = count_on(&grid);
    assert_eq!(count, 1167985679908143);
    count
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    static EXAMPLE_INPUT1: &str = "\
on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";

    static EXAMPLE_INPUT2: &str = "\
on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";

    static EXAMPLE_INPUT3: &str = "\
on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507";

    #[test]
    fn test_intersect_split() {
        // Shaped like:
        // /---\
        // |   |
        // | /-+-\
        // | | | |
        // \-+-/ |
        //   |   |
        //   \---/
        #[rustfmt::skip]
        let a = Range3 { x: (0, 10), y: (0, 10), z: (0, 1) };
        #[rustfmt::skip]
        let b = Range3 { x: (5, 15), y: (5, 15), z: (0, 1) };
        let splits: HashSet<Range3> = intersect_split(a, b).into_iter().collect();
        #[rustfmt::skip]
        let expected: HashSet<Range3> = [
            Range3 { x: (0, 5), y: (0, 5), z: (0, 1) },
            Range3 { x: (5, 10), y: (0, 5), z: (0, 1) },
            Range3 { x: (0, 5), y: (5, 10), z: (0, 1) },
        ].into_iter().collect();
        assert_eq!(splits, expected);

        // Shaped like the previous one, but in 3d
        #[rustfmt::skip]
        let a = Range3 { x: (0, 10), y: (0, 10), z: (0, 10) };
        #[rustfmt::skip]
        let b = Range3 { x: (5, 15), y: (5, 15), z: (5, 15) };
        let splits: HashSet<Range3> = intersect_split(a, b).into_iter().collect();
        #[rustfmt::skip]
        let expected: HashSet<Range3> = [
            Range3 { x: (0, 5), y: (0, 5), z: (0, 5) },
            Range3 { x: (5, 10), y: (0, 5), z: (0, 5) },
            Range3 { x: (0, 5), y: (5, 10), z: (0, 5) },
            Range3 { x: (5, 10), y: (5, 10), z: (0, 5) },
            Range3 { x: (0, 5), y: (0, 5), z: (5, 10) },
            Range3 { x: (5, 10), y: (0, 5), z: (5, 10) },
            Range3 { x: (0, 5), y: (5, 10), z: (5, 10) },
        ].into_iter().collect();
        assert_eq!(splits.len(), expected.len());
        assert_eq!(splits, expected);

        // Shaped like:
        //    /-\
        // /--+-+--\
        // |  | |  |
        // \--+-+--/
        //    \-/
        #[rustfmt::skip]
        let a = Range3 { x: (0, 5), y: (0, 5), z: (0, 1) };
        #[rustfmt::skip]
        let b = Range3 { x: (2, 3), y: (-5, 10), z: (0, 1) };
        let splits: HashSet<Range3> = intersect_split(a, b).into_iter().collect();
        #[rustfmt::skip]
        let expected: HashSet<Range3> = [
            Range3 { x: (0, 2), y: (0, 5), z: (0, 1) },
            Range3 { x: (3, 5), y: (0, 5), z: (0, 1) },
        ].into_iter().collect();
        assert_eq!(splits, expected);

        // Shaped like:
        // /-\ /-\
        // | | | |
        // \-/ \-/
        #[rustfmt::skip]
        let a = Range3 { x: (0, 5), y: (0, 5), z: (0, 1) };
        #[rustfmt::skip]
        let b = Range3 { x: (10, 15), y: (10, 15), z: (0, 1) };
        let splits: HashSet<Range3> = intersect_split(a, b).into_iter().collect();
        #[rustfmt::skip]
        let expected: HashSet<Range3> = [
            Range3 { x: (0, 5), y: (0, 5), z: (0, 1) }
        ].into_iter().collect();
        assert_eq!(splits, expected);
    }

    #[test]
    fn test_reboot() {
        let cuboids = input_generator(EXAMPLE_INPUT1);
        let grid = reboot(&cuboids);
        let count = count_on(&grid);
        assert_eq!(count, 39);

        let mut cuboids = input_generator(EXAMPLE_INPUT2);
        restrict_range_50(&mut cuboids);
        let grid = reboot(&cuboids);
        let count = count_on(&grid);
        assert_eq!(count, 590784);

        let mut cuboids = input_generator(EXAMPLE_INPUT3);
        restrict_range_50(&mut cuboids);
        let grid = reboot(&cuboids);
        let count = count_on(&grid);
        assert_eq!(count, 474140);

        let cuboids = input_generator(EXAMPLE_INPUT3);
        let grid = reboot(&cuboids);
        let count = count_on(&grid);
        assert_eq!(count, 2758514936282235);
    }
}
