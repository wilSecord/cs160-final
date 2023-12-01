use core::panic;
use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Read},
    path::Path,
};

use crate::{UnitedStatesState, shortest_path::NodePath};

pub const FORMAT_NOTE: &str = "This file's format is improvised, but self-documenting. This string MUST be preserved.
All NUMBERs in this file are LITTLE-ENDIAN 32-bit unsigned integers.
This file is composed of zero or more STATE SECTIONS, each of which start with a STATE's name (colon-separated alphanumeric terms), 
followed by a '[' and a NUMBER `count`. The STATE SECTION then holds `count` pairs of NUMBERs; in each pair, the first 
NUMBER is a 'from' and the second NUMBER is a 'to': following these directional links transitively will eventually lead to a 'to'
that doesn't have any more links: this is an artist from the STATE. The path formed by these links is the shortest path to *any* 
artist from the STATE. This file is written in binary instead of csv or another format because in plain-text (even compressed!),
it's much too big to upload to GitHub. The file's content, as described, begins after this double-colon::";

pub fn read_to_nodepaths<P: AsRef<Path>>(
    path: P,
) -> Result<HashMap<UnitedStatesState, HashMap<u32, NodePath<u32>>>, Box<dyn Error>> {
    let result = read(path)?;

    Ok(result.into_iter().map(|(k,v)| {
        let mut new_map: HashMap<u32, NodePath<u32>> = HashMap::new();
        let mut f: VecDeque<_> = v.into_iter().collect();

        while let Some((k,v)) = f.pop_front() {
            if new_map.contains_key(&v) {
                new_map.insert(k, new_map.get(&v).unwrap().clone().with_added(k, 1));
            } else {
                f.push_back((k,v));
            }
        }
        (k, new_map)
    }).collect())
}

pub fn read<P: AsRef<Path>>(
    path: P,
) -> Result<HashMap<UnitedStatesState, HashMap<u32, u32>>, Box<dyn Error>> {
    let mut reader = BufReader::new(File::open(path)?);

    //read the format note & toss it away
    reader.seek_relative(FORMAT_NOTE.len() as i64)?;

    let mut result: HashMap<UnitedStatesState, HashMap<u32, u32>> = HashMap::new();

    //read the name
    loop {
        let mut name_buf = Vec::new();
        if reader.read_until(b'[', &mut name_buf)? == 0 {
            break;
        }
        let mut state_name = String::from_utf8(name_buf)?;

        //remove the deliminator
        state_name.pop();

        let state = UnitedStatesState::try_from_name(&state_name).ok_or("Not a state")?;

        let len = read_u32_le(&mut reader)?;

        eprintln!("{state} {len}");

        result.insert(
            state,
            (0..len)
                .filter_map(|_| {
                    Some((
                        read_utf8_u32(&mut reader).ok().flatten()?,
                        read_utf8_u32(&mut reader).ok().flatten()?,
                    ))
                })
                .collect(),
        );
    }

    Ok(result)
}

fn read_utf8_u32<R: BufRead>(reader: &mut R) -> io::Result<Option<u32>> {
    let mut v = Vec::new();
    reader.read_until(b',', &mut v)?;
    v.pop();
    let s = String::from_utf8(v).ok().map(|x| x.parse().ok()).flatten();
    Ok(s)
}

fn read_u32_le<R: BufRead>(reader: &mut R) -> io::Result<u32> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;

    return Ok(u32::from_le_bytes(buf));
}
