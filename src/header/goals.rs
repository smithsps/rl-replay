use nom::*;
use header::primitives::*;

#[derive(Debug)]
pub struct Goal<'a> {
    frame: u32,
    player: &'a str,
    team: u32
}

named!(pub goals_array <Vec<Goal>>,
    do_parse!(
        tag!(b"\x06\x00\x00\x00Goals\0") >>
        array_size: array_property >>

        goals: length_count!(le_u32, do_parse!(
            tag!(b"\x06\x00\x00\x00frame\0") >>
            frame: int_property >>

            tag!(b"\x0B\x00\x00\x00PlayerName\0") >>
            player_name: str_property >>

            tag!(b"\x0B\x00\x00\x00PlayerTeam\0") >>
            team: int_property >>

            // Unknown why there is a 'None' delimiting
            str_none >>

            (Goal {
                frame: frame,
                player: player_name,
                team: team
            })
        )) >>
        (goals)
    )
);