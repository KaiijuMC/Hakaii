use std::path::Path;

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
        let mut count = 0;
        let mut nulled = 0;
        let mut todelete = Vec::new();

        // REGION PART
        let filename = region_dir.join(&name);
        let mut rregion = load_file(&filename)
            .expect(&format!("Cannot read region file {:?}", filename));

        for i in 0..1024 {
            if let Some(ref chunk) = rregion.chunks[i] {
                let val: Data = fastnbt::from_bytes(chunk.raw_chunk.as_slice())
                    .expect(&format!("Cannot decode {:?}, {:?} data!", rregion, chunk));
                let time = val.inhabited_time;
                if time < duration {
                    count += 1;
                    todelete.push(i);
                    rregion.chunks[i] = None;
                }
            } else {
                nulled += 1;
            }
        }
        rregion.write_linear(region_dir.to_str().unwrap(), compression_level)
            .expect(&format!("Cannot write {:?}", filename));

        // ENTITIES PART
        let filename = entities_dir.join(&name);
        let eregion = load_file(&filename);
        if let Some(mut region) = eregion {
            for i in &todelete {
                region.chunks[*i] = None;
            }
            region.write_linear(entities_dir.to_str().unwrap(), compression_level)
                .expect(&format!("Cannot write {:?}", filename));
        }

        // POI PART
        let filename = poi_dir.join(&name);
        let pregion = load_file(&filename);
        if let Some(mut region) = pregion {
            for i in &todelete {
                region.chunks[*i] = None;
            }
            region.write_linear(poi_dir.to_str().unwrap(), compression_level)
                .expect(&format!("Cannot write {:?}", filename));
        }

        println!(
            "[REGION {:>4} {:>4}] Deleted {:>4} chunks, {:>4} ungenerated chunks, keeping {:>4} chunks",
            rregion.region_x,
            rregion.region_z,
            count,
            nulled,
            1024 - count - nulled
        );
    }
}

fn load_file(filename: &Path) -> Option<Region> {
    linearify::open_linear(filename.to_str().unwrap()).ok()
}
