#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum UnitedStatesState {
    Guam,
    NorthernMarianaIslands,
    Virginia,
    Maryland,
    Connecticut,
    Arizona,
    Colorado,
    Arkansas,
    Florida,
    Alabama,
    California,
    Georgia,
    Hawaii,
    Iowa,
    Idaho,
    Illinois,
    Indiana,
    Kansas,
    Kentucky,
    Louisiana,
    Massachusetts,
    Maine,
    Michigan,
    Minnesota,
    Missouri,
    Mississippi,
    Montana,
    NorthCarolina,
    NorthDakota,
    Nebraska,
    NewHampshire,
    NewJersey,
    NewMexico,
    Nevada,
    NewYork,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Washington,
    Wisconsin,
    WestVirginia,
    Wyoming,
    AmericanSamoa,
    PuertoRico,
    UnitedStatesMinorOutlyingIslands,
    Alaska,
    WashingtonDC,
    Delaware,
}

macro_rules! enum_parsable {
    { $typ:path => { $( $a:path => $b:literal ),* } }   => {
        impl From<$typ> for &str {
            fn from(val: $typ) -> Self {
                match val {
                    $(
                       $a => $b,
                    )*
                }
            }
        }

        impl std::fmt::Display for $typ {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(Into::<&str>::into(*self))
            }
        }

        impl std::str::FromStr for $typ {
            type Err = String;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match value {
                    $(
                        $b => Ok($a),
                     )*
                     v => Err(format!("Could not parse state from {}", v)),
                }
            }
        }

        impl $typ {
            pub fn representations() -> Vec<&'static str> {
                vec![
                    $( $b, )*
                ]
            }

            pub fn items() -> Vec<$typ> {
                vec![
                    $( $a, )*
                ]
            }
            pub fn name(&self) -> &'static str {
                match &self {
                    $( $a => stringify!($a), )*
                }
            }

            pub fn try_from_representation(v: &str) -> Option<Self> {
                v.parse().ok()
            }

            pub fn try_from_name(n: &str) -> Option<Self> {
                match n {
                    $( stringify!($a) => Some($a), )*
                    _ => None
                }
            }

        }

    };
}


enum_parsable! {
    UnitedStatesState => {
        UnitedStatesState::Guam => "88",
        UnitedStatesState::NorthernMarianaIslands => "159",
        UnitedStatesState::Virginia => "306",
        UnitedStatesState::Maryland => "261",
        UnitedStatesState::Connecticut => "268",
        UnitedStatesState::Arizona => "265",
        UnitedStatesState::Colorado => "267",
        UnitedStatesState::Arkansas => "264",
        UnitedStatesState::Florida => "271",
        UnitedStatesState::Alabama => "263",
        UnitedStatesState::California => "266",
        UnitedStatesState::Georgia => "272",
        UnitedStatesState::Hawaii => "273",
        UnitedStatesState::Iowa => "274",
        UnitedStatesState::Idaho => "275",
        UnitedStatesState::Illinois => "276",
        UnitedStatesState::Indiana => "277",
        UnitedStatesState::Kansas => "278",
        UnitedStatesState::Kentucky => "279",
        UnitedStatesState::Louisiana => "280",
        UnitedStatesState::Massachusetts => "281",
        UnitedStatesState::Maine => "282",
        UnitedStatesState::Michigan => "283",
        UnitedStatesState::Minnesota => "284",
        UnitedStatesState::Missouri => "285",
        UnitedStatesState::Mississippi => "286",
        UnitedStatesState::Montana => "287",
        UnitedStatesState::NorthCarolina => "288",
        UnitedStatesState::NorthDakota => "289",
        UnitedStatesState::Nebraska => "290",
        UnitedStatesState::NewHampshire => "291",
        UnitedStatesState::NewJersey => "292",
        UnitedStatesState::NewMexico => "293",
        UnitedStatesState::Nevada => "294",
        UnitedStatesState::NewYork => "295",
        UnitedStatesState::Ohio => "296",
        UnitedStatesState::Oklahoma => "297",
        UnitedStatesState::Oregon => "298",
        UnitedStatesState::Pennsylvania => "299",
        UnitedStatesState::RhodeIsland => "300",
        UnitedStatesState::SouthCarolina => "301",
        UnitedStatesState::SouthDakota => "302",
        UnitedStatesState::Tennessee => "303",
        UnitedStatesState::Texas => "304",
        UnitedStatesState::Utah => "305",
        UnitedStatesState::Vermont => "307",
        UnitedStatesState::Washington => "308",
        UnitedStatesState::Wisconsin => "309",
        UnitedStatesState::WestVirginia => "310",
        UnitedStatesState::Wyoming => "311",
        UnitedStatesState::AmericanSamoa => "4",
        UnitedStatesState::PuertoRico => "172",
        UnitedStatesState::UnitedStatesMinorOutlyingIslands => "223",
        UnitedStatesState::Alaska => "262",
        UnitedStatesState::WashingtonDC => "269",
        UnitedStatesState::Delaware => "270"
    } 
}
