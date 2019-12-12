use std::fmt::Debug;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct CoordAndSteps {
    coord: Coord,
    steps: usize
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(item: &str) -> Result<Direction, String> {
        use Direction::*;

        match item {
            "U" => Ok(Up),
            "R" => Ok(Right),
            "D" => Ok(Down),
            "L" => Ok(Left),
            _ => Err(String::from("Invalid direction provided"))
        }
    }
}

// credit to reddit thread: https://www.reddit.com/r/rust/comments/31syce/using_iterators_to_find_the_index_of_the_min_or/
fn min_index(array: &[i32]) -> usize {
    let mut i = 0;

    for (j, &value) in array.iter().enumerate() {
        if value < array[i] {
            i = j;
        }
    }

    i
}

// Found on stack overflow
// Return the first char of a string and the rest
fn car_cdr(s: &str) -> (&str, &str) {
    for i in 1..5 {
        let r = s.get(0..i);
        match r {
            Some(x) => return (x, &s[i..]),
            None => (),
        }
    }

    (&s[0..0], s)
}


fn wire_path(path: String) -> HashMap<String, i32> {
    println!("Start wire path");
    let mut positions: HashMap<String, i32> = HashMap::new();
    let mut steps = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for item in path.split(",") {
        let (direction_string, distance_string) = car_cdr(item);
        let distance = distance_string.parse::<i32>().unwrap();
        let direction = direction_string.parse::<Direction>();
        use Direction::*;

        match direction {
            Ok(Up) => {
                for i in 0..distance {
                    positions.insert(format!("{}_{}", x, y + i), steps + i);
                }
                y += distance;
            },
            Ok(Right) => {
                for i in 0..distance {
                    positions.insert(format!("{}_{}", x + i, y), steps + i);
                }
                x += distance;
            },
            Ok(Down) => {
                for i in 0..distance {
                    positions.insert(format!("{}_{}", x, y - i), steps + i);
                }
                y -= distance;
            },
            Ok(Left) => {
                for i in 0..distance {
                    positions.insert(format!("{}_{}", x - i, y), steps + i);
                }
                x -= distance;
            },
            Err(error_message) => {
                println!("Error {}, input: {}", error_message, item);
            }
        }
        steps += distance;
    }

    println!("End wire path");
    positions
}

fn get_closest_collision_distance(collisions: &Vec<CoordAndSteps>) -> i32 {
    if collisions.len() == 0 {
        return 0
    }

    println!("found {:?} collisions", collisions.len());
    let collisions_distances: Vec<i32> = collisions
        .iter()
        .map(|w| w.coord.x.abs() + w.coord.y.abs())
        .collect();
    let min_index_found = min_index(&collisions_distances);
    return collisions_distances[min_index_found];
}

fn get_paths_min_collision_distance(path1: String, path2: String) -> Vec<i32> {
    let positions_1 = wire_path(path1);
    let positions_2 = wire_path(path2);
    let mut collisions: Vec<CoordAndSteps> = vec![];

    for w in positions_1.iter() {
        if w.0 != "0_0" {
            let entry = positions_2.get(w.0);
            match entry {
                Some(item)=> {
                    let coords: Vec<&str> = w.0.split("_").collect();
                    collisions.push(CoordAndSteps {
                        coord: Coord {
                            x: coords[0].parse::<i32>().unwrap(),
                            y: coords[1].parse::<i32>().unwrap(),
                        },
                        steps: (item + w.1) as usize
                    });
                },
                None => {}
            }
        }
    }

    println!("found {:?} collisions", collisions.len());

    vec![
        get_closest_collision_distance(&collisions),
        get_closest_collision_steps(&collisions)
    ]
}

fn get_closest_collision_steps(collisions: &Vec<CoordAndSteps>) -> i32 {
    if collisions.len() == 0 {
        return 0
    }

    collisions
        .iter()
        .map(|w| w.steps)
        .min()
        .unwrap() as i32
}

pub fn main() {
    let closest_collision = get_paths_min_collision_distance(
        String::from("R992,U284,L447,D597,R888,D327,R949,U520,R27,U555,L144,D284,R538,U249,R323,U297,R136,U838,L704,D621,R488,U856,R301,U539,L701,U363,R611,D94,L734,D560,L414,U890,R236,D699,L384,D452,R702,D637,L164,U410,R649,U901,L910,D595,R339,D346,R959,U777,R218,D667,R534,D762,R484,D914,L25,U959,R984,D922,R612,U999,L169,D599,L604,D357,L217,D327,L730,D949,L565,D332,L114,D512,R460,D495,L187,D697,R313,U319,L8,D915,L518,D513,R738,U9,R137,U542,L188,U440,R576,D307,R734,U58,R285,D401,R166,U156,L859,U132,L10,U753,L933,U915,R459,D50,R231,D166,L253,U844,R585,D871,L799,U53,R785,U336,R622,D108,R555,D918,L217,D668,L220,U738,L997,D998,R964,D456,L54,U930,R985,D244,L613,D116,L994,D20,R949,D245,L704,D564,L210,D13,R998,U951,L482,U579,L793,U680,L285,U770,L975,D54,R79,U613,L907,U467,L256,D783,R883,U810,R409,D508,L898,D286,L40,U741,L759,D549,R210,U411,R638,D643,L784,U538,L739,U771,L773,U491,L303,D425,L891,U182,R412,U951,L381,U501,R482,D625,R870,D320,L464,U555,R566,D781,L540,D754,L211,U73,L321,D869,R994,D177,R496,U383,R911,U819,L651,D774,L591,U666,L883,U767,R232,U822,L499,U44,L45,U873,L98,D487,L47,U803,R855,U256,R567,D88,R138,D678,L37,U38,R783,U569,L646,D261,L597,U275,L527,U48,R433,D324,L631,D160,L145,D128,R894,U223,R664,U510,R756,D700,R297,D361,R837,U996,L769,U813,L477,U420,L172,U482,R891,D379,L329,U55,R284,U155,L816,U659,L671,U996,R997,U252,R514,D718,L661,D625,R910,D960,L39,U610,R853,U859,R174,U215,L603,U745,L587,D736,R365,U78,R306,U158,L813,U885,R558,U631,L110,D232,L519,D366,R909,D10,R294"),
        String::from("L1001,D833,L855,D123,R36,U295,L319,D700,L164,U576,L68,D757,R192,D738,L640,D660,R940,D778,R888,U772,R771,U900,L188,D464,L572,U184,R889,D991,L961,U751,R560,D490,L887,D748,R37,U910,L424,D401,L385,U415,L929,U193,R710,D855,L596,D323,L966,D505,L422,D139,L108,D135,R737,U176,R538,D173,R21,D951,R949,D61,L343,U704,R127,U468,L240,D834,L858,D127,R328,D863,R329,U477,R131,U864,R997,D38,R418,U611,R28,U705,R148,D414,R786,U264,L785,D650,R201,D250,R528,D910,R670,U309,L658,U190,R704,U21,R288,D7,R930,U62,R782,U621,R328,D725,R305,U700,R494,D137,R969,U142,L867,U577,R300,U162,L13,D698,R333,U865,R941,U796,L60,U902,L784,U832,R78,D578,R196,D390,R728,D922,R858,D994,L457,U547,R238,D345,R329,D498,R873,D212,R501,U474,L657,U910,L335,U133,R213,U417,R698,U829,L2,U704,L273,D83,R231,D247,R675,D23,L692,D472,L325,D659,L408,U746,L715,U395,L596,U296,R52,D849,L713,U815,R684,D551,L319,U768,R176,D182,R557,U731,R314,D543,L9,D256,R38,D809,L567,D332,R375,D572,R81,D479,L71,U968,L831,D247,R989,U390,R463,D576,R740,D539,R488,U367,L596,U375,L763,D824,R70,U448,R979,D977,L744,D379,R488,D671,L516,D334,L542,U517,L488,D390,L713,D932,L28,U924,L448,D229,L488,D501,R19,D910,L979,D411,R711,D824,L973,U291,R794,D485,R208,U370,R655,U450,L40,D804,L374,D671,R962,D829,L209,U111,L84,D876,L832,D747,L733,D560,L702,D972,R188,U817,L111,U26,L492,U485,L71,D59,L269,D870,L152,U539,R65,D918,L932,D260,L485,U77,L699,U254,R924,U643,L264,U96,R395,D917,R360,U354,R101,D682,R854,U450,L376,D378,R872,D311,L881,U630,R77,D766,R672")
    );
    println!("{:?}", closest_collision);
}

#[test]
pub fn test_direction_parse() {
    let direction_str_1 = "U";
    let direction_str_2 = "R";
    let direction_str_3 = "D";
    let direction_str_4 = "L";
    let direction_1 = direction_str_1.parse::<Direction>();
    let direction_2 = direction_str_2.parse::<Direction>();
    let direction_3 = direction_str_3.parse::<Direction>();
    let direction_4 = direction_str_4.parse::<Direction>();

    assert_eq!(Direction::Up, direction_1.unwrap());
    assert_eq!(Direction::Right, direction_2.unwrap());
    assert_eq!(Direction::Down, direction_3.unwrap());
    assert_eq!(Direction::Left, direction_4.unwrap());
}

#[test]
pub fn test_wire_collisions_min_distance() {
    let closest_collision_1 = get_paths_min_collision_distance(
        String::from("R8,U5,L5,D3"),
        String::from("U7,R6,D4,L4")
    );
    let closest_collision_2 = get_paths_min_collision_distance(
        String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
        String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
    );

    assert_eq!(6, closest_collision_1[0]);
    assert_eq!(30, closest_collision_1[1]);
    assert_eq!(135, closest_collision_2[0]);
    assert_eq!(410, closest_collision_2[1]);
}