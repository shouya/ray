use common::*;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Default, Clone, Debug)]
pub struct ObjModel {
  pub o: Option<String>, // name
  pub v: Vec<V3>,
  pub f: Vec<Vec<usize>>,
}

impl ObjModel {
  pub fn from_file<P: AsRef<Path>>(path: P) -> Option<ObjModel> {
    use std::io::BufRead;

    let file = File::open(path).ok()?;
    let reader = BufReader::new(file);

    let mut obj = Self::default();

    for line in reader.lines() {
      let line = line.unwrap();
      let elem: Vec<_> = line.split(' ').collect();
      match elem[0] {
        "#" => {}
        "o" => {
          obj.o = Some(line[2..line.len()].into());
        }
        "v" => {
          let v: Vec<f32> = elem[1..].iter().map(|x| x.parse().unwrap()).collect();
          obj.v.push(V3([v[0], v[1], v[2]]));
        }
        "f" => {
          let f: Vec<usize> = elem[1..]
            .iter()
            .map(|x| x.parse().unwrap())
            .map(|x: usize| x - 1) // .obj file's array starts at 1 :)
            .collect();
          obj.f.push(f);
        }
        "s" => {
          // ignore
        }
        _ => {
          // unknown properties
        }
      }
    }

    Some(obj)
  }
}
