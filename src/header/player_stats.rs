use nom::*;
use header::primitives::*;

#[derive(Debug)]
pub struct PlayerStats<'a> {
    name: &'a str,
    is_bot: bool,
    platform: &'a [u8],
    online_platform: &'a str,
    online_id: &'a [u8],
    team: u32,
    score: u32,
    goals: u32,
    assists: u32,
    saves: u32,
    shots: u32
}

named!(pub playerstats_array <Vec<PlayerStats>>,
    do_parse!(
        tag!(b"\x0C\x00\x00\x00PlayerStats\0") >>
        array_size: array_property >>

        stats: length_count!(le_u32, do_parse!(
            tag!(b"\x05\x00\x00\x00Name\0") >>
            player_name: str_property >>

            tag!(b"\x09\x00\x00\x00Platform\0") >>
            platform: byte_property >>

            tag!(b"\x0F\x00\x00\x00OnlinePlatform\0") >>
            online_platform: raw_string >>

            tag!(b"\x09\x00\x00\x00OnlineID\0") >>
            online_id: qword_property >>

            tag!(b"\x05\x00\x00\x00Team\0") >>
            team: int_property >>

            tag!(b"\x06\x00\x00\x00Score\0") >>
            score: int_property >>

            tag!(b"\x06\x00\x00\x00Goals\0") >>
            goals: int_property >>

            tag!(b"\x08\x00\x00\x00Assists\0") >>
            assists: int_property >>

            tag!(b"\x06\x00\x00\x00Saves\0") >>
            saves: int_property >>

            tag!(b"\x06\x00\x00\x00Shots\0") >>
            shots: int_property >>

            tag!(b"\x05\x00\x00\x00bBot\0") >>
            is_bot: bool_property >>

            str_none >>

            (PlayerStats {
                name: player_name,
                is_bot: is_bot,
                platform: platform,
                online_platform: online_platform,
                online_id: online_id,
                team: team,
                score: score,
                goals: goals,
                assists: assists,
                saves: saves,
                shots: shots
            })
        )) >>
        (stats)
    )
);