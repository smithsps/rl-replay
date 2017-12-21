//Header Parsing

use nom::*;

mod highlights;
mod metadata;
mod player_stats;
mod goals;
mod primitives;

use header::highlights::{Highlight, highlight_array};
use header::metadata::{MetaInfo, meta_info};
use header::player_stats::{PlayerStats, playerstats_array};
use header::goals::{Goal, goals_array};
use header::primitives::*;


//named!(str_None, tag!(b"\x05\x00\x00\x00None\0"));


#[derive(Debug)]
pub struct ReplayHeader<'a> {
    //crc: &'a [u8],
    version_major: u32,
    version_minor: u32,

    team_size: u32,
    team_0_score: u32,
    team_1_score: u32,

    goals: Vec<Goal<'a>>,
    highlights: Vec<Highlight<'a>>,
    stats: Vec<PlayerStats<'a>>,
    meta: MetaInfo<'a>
}


named!(pub get_header<(ReplayHeader)>,
    do_parse!(
        //crc: take!(4) >>
        version_major: dbg_dmp!(le_u32) >>
        version_minor: dbg_dmp!(le_u32) >>
        unknown_perhaps_version_detail: take!(4) >>
        tag!(b"\x18\x00\x00\x00TAGame.Replay_Soccar_TA\0") >>

        tag!(b"\x09\x00\x00\x00TeamSize\0") >>
        team_size: int_property >>

        opt!(tag!(b"\x12\x00\x00\x00PrimaryPlayerTeam\0")) >>
        primary_player_team: opt!(int_property) >>

        tag!(b"\x0B\x00\x00\x00Team0Score\0") >>
        team_0_score: int_property >>

        tag!(b"\x0B\x00\x00\x00Team1Score\0") >>
        team_1_score: int_property >>

        goals: dbg!(goals_array) >>

        highlights: dbg!(highlight_array) >>

        stats: dbg!(playerstats_array) >>

        meta: dbg_dmp!(meta_info) >>

        //rest: rest >>
        (ReplayHeader {
            //crc: crc,
            version_major: version_major,
            version_minor: version_minor,

            team_size: team_size,
            team_0_score: team_0_score,
            team_1_score: team_1_score,

            goals: goals,
            highlights: highlights,
            stats: stats,
            meta: meta
        })
    )
);