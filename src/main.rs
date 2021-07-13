use libpdb;
use anyhow;

use std::env;

fn main() -> anyhow::Result<()> {
    let filename = env::args().nth(1).unwrap();
    let pdb = libpdb::read_pdb(&filename)?;

    let pdb_id = if let Some(id) = pdb.identifier() {
        id.to_owned().to_lowercase()
    } else {filename.split_at(4).0.to_lowercase()};
    
    let combis = get_combi_chains(&pdb);

    // one_two [["A", "B"]]
    if combis.len() == 1 && combis[0].len() == 2 {
        ones_split_chains(&pdb, &combis, &pdb_id)?;
    } // same_two_two [["A", "B"], ["A", "B"]]
    else if combis.len() == 2 && combis[0].len() == 2 && combis[0] == combis[1] {
        ones_split_chains(&pdb, &combis, &pdb_id)?;
    } // two [["A", "B", "C"], ["D", "E"]]
    else if combis.len() == 2 && combis[0] != combis[1] {
        twos_split_chains(&pdb, &combis, &pdb_id)?;
    }

    Ok(())
}

fn get_combi_chains(pdb: &libpdb::PDB) -> Vec<Vec<&str>> {
    let mut combis = Vec::new();

    for (_num, remark) in pdb.remarks().filter(|(num, remark)| {
            *num as u64 == 350 && remark.contains("APPLY THE FOLLOWING TO CHAINS:")
        }) {
            let mut pair = Vec::<&str>::new();
            match remark.strip_prefix("APPLY THE FOLLOWING TO CHAINS: ") {
                Some(s) => pair = s.split(", ").collect(),
                None => pair.push(""),
            }
            combis.push(pair);
        }
        combis
}


/// ex. combis = [["A", "B"]] or [["A", "B"], ["A", "B"]]
fn ones_split_chains(pdb: &libpdb::PDB, combis: &Vec<Vec<&str>>, pdb_id: &str) -> anyhow::Result<()>
{
    for (i, chain_id) in combis[0].iter().enumerate() {
        let mut new_pdb = libpdb::PDB::new();
        pdb.atoms()
        .filter(|atom| chain_id == atom.chain_id())
        .for_each(|atom| new_pdb.add_atom(atom.to_owned()));

        libpdb::save_pdb_atom(
            new_pdb,
            format!("./output/{}_{}.pdb", pdb_id, i).as_str()
        )?;

        println!("save ./output/{}_{}.pdb", pdb_id, i);
    }
    Ok(())
}

/// ex. combis = [["A", "B"], ["C", "D"]]
fn twos_split_chains(pdb: &libpdb::PDB, combis: &Vec<Vec<&str>>, pdb_id: &str) -> anyhow::Result<()>
{
    for (i, pair) in combis.iter().enumerate() {
        let mut new_pdb = libpdb::PDB::new();
        pdb.atoms()
        .filter(|atom| pair.contains(&atom.chain_id().as_str()))
        .for_each(|atom| new_pdb.add_atom(atom.to_owned()));

        libpdb::save_pdb_atom(
            new_pdb,
            format!("./output/{}_{}.pdb", pdb_id, i).as_str()
        )?;

        println!("save ./output/{}_{}.pdb", pdb_id, i);
    }
    Ok(())
}