use core::panic;
use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Read},
    path::Path,
};

use crate::{shortest_path::NodePathSystem, UnitedStatesState};

pub const FORMAT_NOTE: &str = "This file's format is improvised, but self-documenting. This string MUST be preserved.
All NUMBERs in this file are LITTLE-ENDIAN 32-bit unsigned integers.
This file is composed of zero or more STATE SECTIONS, each of which start with a STATE's name (colon-separated alphanumeric terms), 
followed by a '[' and a NUMBER `count`. The STATE SECTION then holds `count` pairs of NUMBERs; in each pair, the first 
NUMBER is a identifier of the current node, and the second NUMBER is a 0-based index link into the current STATE SECTION: 
following these directional links transitively will eventually lead to a 'to'
that links to itself: this is an artist from the STATE. The path formed by these links is the shortest path to *any* 
artist from the STATE. This file is written in binary instead of csv or another format because in plain-text (even compressed!),
it's much too big to upload to GitHub. The file's content, as described, begins after this double-colon::";

pub fn read_nodesystem<P: AsRef<Path>>(
    path: P,
) -> Result<HashMap<UnitedStatesState, NodePathSystem>, Box<dyn Error>> {
    read_into_collection::<_, _>(path)
}

pub struct AdHocShortestPathFileFormatReader {
    reader: BufReader<File>,
}

impl Iterator for AdHocShortestPathFileFormatReader {
    type Item = (UnitedStatesState, NodePathSystem);

    fn next(&mut self) -> Option<Self::Item> {
        let reader = &mut self.reader;

        let mut name_buf = Vec::new();
        if reader.read_until(b'[', &mut name_buf).unwrap_or(0) == 0 {
            return None;
        }
        let mut state_name = String::from_utf8(name_buf).unwrap();

        //remove the deliminator
        state_name.pop();

        let state = UnitedStatesState::try_from_name(&state_name).unwrap();

        let len = read_u32_le(reader).unwrap();

        let mut pathsystem = NodePathSystem::new_with_capacity((len + 1).try_into().unwrap());

        for i in 0..len {
            pathsystem.insert_link(
                i,
                read_u32_le(reader).unwrap().try_into().unwrap(),
                read_u32_le(reader).unwrap().try_into().unwrap(),
            );
        }

        Some((state, pathsystem))
    }
}

pub fn read_into_collection<
    P: AsRef<Path>,
    I: FromIterator<(UnitedStatesState, NodePathSystem)>,
>(
    path: P,
) -> Result<I, Box<dyn Error>> {
    read(path).map(|x| x.collect())
}

pub fn read<
    P: AsRef<Path>
>(
    path: P,
) -> Result<AdHocShortestPathFileFormatReader, Box<dyn Error>> {
    let mut reader = BufReader::new(File::open(path)?);
    //read the format note & toss it away
    reader.seek_relative(FORMAT_NOTE.len() as i64)?;

    Ok(AdHocShortestPathFileFormatReader { reader })
}

fn read_utf8_u32<R: BufRead>(reader: &mut R) -> io::Result<u32> {
    let mut v = Vec::new();
    reader.read_until(b',', &mut v)?;
    v.pop();
    let s = String::from_utf8(v).unwrap().parse().unwrap();
    Ok(s)
}

fn read_u32_le<R: BufRead>(reader: &mut R) -> io::Result<u32> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;

    return Ok(u32::from_le_bytes(buf));
}
