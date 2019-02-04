use common::*;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Default, Clone, Debug)]
pub struct ObjModel {
    pub o: Option<String>, // name
    pub v: Vec<V3>,
    pub vn: Vec<V3>,
    pub f: Vec<Vec<(usize, usize)>>,
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
                "#" => {
                    // comment
                }
                "o" => {
                    obj.o = Some(line[2..line.len()].into());
                }
                "v" => {
                    obj.v.push(Self::parse_v3(&elem[1..]));
                }
                "vn" => {
                    obj.vn.push(Self::parse_v3(&elem[1..]));
                }
                "f" => {
                    obj.f.push(Self::parse_face(&elem[1..]));
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

    fn parse_v3(s: &[&str]) -> V3 {
        assert!(s.len() == 3);
        let v: Vec<f32> = s.iter().map(|x| x.parse().unwrap()).collect();
        V3([v[0], v[1], v[2]])
    }

    fn parse_face(s: &[&str]) -> Vec<(usize, usize)> {
        let parse_idx = |x: &str| x.parse::<usize>().unwrap() - 1;
        s.iter()
            .map(|x| x.split(r"//").collect())
            .map(|x: Vec<_>| x.into_iter().map(parse_idx).collect())
            .map(|x: Vec<_>| (x[0], x[1]))
            .collect()
    }
}
