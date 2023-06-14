use std::{fs, path::Path};

use linearify::{self, Region};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    pub inhabited_time: i64
}

pub fn clean_regions(dirname: &str, duration: i64, compression_level: i32, files: &Vec<String>) {
    let region_dir = Path::new(dirname).join("region");
    let entities_dir = Path::new(dirname).join("entities");
    let poi_dir = Path::new(dirname).join("poi");

    for name in files {
        let mut nulled = 0;
        let mut todelete = Vec::new();
        let mut todelete_count = 0;

        // LOAD FILES
        let rfilename = region_dir.join(&name);
        let mut rregion = load_file(&rfilename)
            .expect(&format!("Cannot read region file {:?}", rfilename));
        let efilename = entities_dir.join(&name);
        let eregion = load_file(&efilename);
        let pfilename = poi_dir.join(&name);
        let pregion = load_file(&pfilename);

        // REGION PART
        for i in 0..1024 {
            if let Some(ref chunk) = rregion.chunks[i] {
                let val: Data = fastnbt::from_bytes(chunk.raw_chunk.as_slice())
                    .expect(&format!("Cannot decode {:?}, {:?} data!", rregion, chunk));
                let time = val.inhabited_time;
                if time < duration {
                    todelete.push(i);
                    todelete_count += 1;
                    rregion.chunks[i] = None;
                }
            } else {
                nulled += 1;
            }
        }

        println!(
            "[REGION {:>4} {:>4}] Deleting {:>4} chunks, {:>4} null chunks, keeping {:>4} chunks",
            rregion.region_x,
            rregion.region_z,
            todelete_count,
            nulled,
            1024 - todelete_count - nulled
        );

        if todelete_count + nulled == 1024 {
            fs::remove_file(&rfilename)
                .expect(&format!("Cannot delete file {:?}", rfilename));
            if eregion.is_some() {
                fs::remove_file(&efilename)
                    .expect(&format!("Cannot delete file {:?}", efilename));
            }
            if pregion.is_some() {
                fs::remove_file(&pfilename)
                    .expect(&format!("Cannot delete file {:?}", pfilename));
            }
            continue;
        }

        rregion.write_linear(region_dir.to_str().unwrap(), compression_level)
            .expect(&format!("Cannot write {:?}", rfilename));

        // ENTITIES PART
        if let Some(mut region) = eregion {
            for i in &todelete {
                region.chunks[*i] = None;
            }
            region.write_linear(entities_dir.to_str().unwrap(), compression_level)
                .expect(&format!("Cannot write {:?}", rfilename));
        }

        // POI PART
        if let Some(mut region) = pregion {
            for i in &todelete {
                region.chunks[*i] = None;
            }
            region.write_linear(poi_dir.to_str().unwrap(), compression_level)
                .expect(&format!("Cannot write {:?}", pfilename));
        }
    }
}

fn load_file(filename: &Path) -> Option<Region> {
    linearify::open_linear(filename.to_str().unwrap()).ok()
}
