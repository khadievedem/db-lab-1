// mod.rs

pub mod randi64 {
    use core::arch::global_asm;

    global_asm!(include_str!("random.asm"));

    #[allow(dead_code)]
    extern "sysv64" {
        fn system_rand(seed: i64, module: i32) -> i64;
    }

    // extern "efiapi" {
    //     fn efi_rand(seed: i64) -> i64;
    // }
    // extern "win64" {
    //     fn win64_rand(seed: i64) -> i64;
    // }

    #[allow(dead_code)]
    pub fn asm_random(module: i32) -> u32 {
        let seed = rand::random::<i64>();
        let prime: u32 = 289035269;
        let res: i64;

        unsafe {
            res = system_rand(seed, module);
        }
        res as u32 % prime + 1
    }
}

pub mod tablmgr {
    use std::fs::File;
    use std::io::{Read, Write};
    use std::path::Path;

    use super::tabltools;

    #[allow(dead_code)]
    pub fn add(_fldr: String, _tbl: String, data: String) -> Result<usize, std::io::Error> {
        let path = format!("../{}/{}.txt", _fldr, _tbl);
        let output = Path::new(path.as_str());

        let mut file = match File::options().read(true).append(true).open(output) {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let new_data = data.trim().replace(" ", ",");
        if !tabltools::uniq_check(&mut file, &new_data) {
            return Ok(0); // means that 0 bytes was writen
        }

        file.write(new_data.as_bytes())
    }

    #[allow(dead_code)]
    pub fn edit(
        _fldr: String,
        _tbl: String,
        id: u32,
        new_data: String,
    ) -> Result<usize, std::io::Error> {
        let path = format!("../{}/{}.txt", _fldr, _tbl);
        let output = Path::new(path.as_str());

        let mut src = match File::open(output) {
            Ok(src) => src,
            Err(e) => return Err(e),
        };

        let mut old_lines = String::new();
        if let Err(e) = src.read_to_string(&mut old_lines) {
            eprintln!("[ERROR] {}", e);
        }

        if !tabltools::uniq_check(&mut src, &new_data.replace(" ", ",")) {
            return Ok(0); // means that file opened but no lines edited
        }

        drop(src);

        let new_lines = old_lines.replace(
            old_lines
                .split('\n')
                .collect::<Vec<_>>()
                .get(id as usize)
                .expect("[ERROR] element with this id doesn't exist"),
            &new_data.replace(" ", ","),
        );
        let mut dst = match File::open(output) {
            Ok(dst) => dst,
            Err(e) => return Err(e),
        };

        dst.write(new_lines.as_bytes())
    }

    #[allow(dead_code)]
    pub fn del(_fldr: String, _tbl: String, id: u64) -> String {
        let path = format!("../{}/{}.txt", _fldr, _tbl);
        let output = Path::new(path.as_str());

        let mut src = File::open(output).expect("[ERROR] unable to open file");
        let mut old_lines = String::new();
        if let Err(e) = src.read_to_string(&mut old_lines) {
            eprintln!("[ERROR] {}", e);
        }
        drop(src);

        let deleted_line = format!(
            "{}\n",
            old_lines
                .split('\n')
                .collect::<Vec<_>>()
                .get(id as usize)
                .expect("[ERROR] element with this id doesn't exist")
        );
        let data = old_lines.replace(deleted_line.as_str(), "");

        let mut dst = File::open(output).expect("[ERROR] unable to open file");
        dst.write(data.as_bytes())
            .expect("[ERROR] unable write to the file");

        deleted_line
    }

    #[allow(dead_code)]
    pub fn print(_fldr: String, _tbl: String, id: u32) -> String {
        let path = format!("../{}/{}.txt", _fldr, _tbl);
        let output = Path::new(path.as_str());

        let mut src = File::open(output).expect("[ERROR] unable to open file");
        let mut lines = String::new();
        if let Err(e) = src.read_to_string(&mut lines) {
            eprintln!("[ERROR] {}", e);
        }

        lines
            .split('\n')
            .collect::<Vec<_>>()
            .get(id as usize)
            .expect("[ERROR] element with this id doesn't exist")
            .replace(",", " ")
            .to_string()
    }

    #[allow(dead_code)]
    pub fn create(name: String, access: char) -> File {
        let path = format!("../generated_tables/{}.txt", name);
        let new_tbl = Path::new(path.as_str());

        match std::fs::remove_file(new_tbl) {
            Ok(()) => eprintln!("[INFO] File {path} deleted!"),
            Err(_) => eprint!(""),
        };

        let created_file = match access {
            'r' => File::options().read(true).create(true).open(new_tbl),
            'w' => File::options()
                .read(true)
                .write(true)
                .create(true)
                .open(new_tbl),
            'a' => File::options()
                .read(true)
                .append(true)
                .create(true)
                .open(new_tbl),
            _ => File::open("failed"),
        };
        created_file.expect("[ERROR] failed to open file")
    }

    pub fn delete(name: String) -> Result<(), std::io::Error> {
        let path = format!("../generated_tables/{}", name);
        let del_tbl = Path::new(path.as_str());

        std::fs::remove_file(del_tbl)
    }

    pub fn clean() -> Result<(), std::io::Error> {
        for file_name in ["testing_table", ".temp"] {
            if let Err(e) =
                std::fs::remove_file(format!("../generated_tables/{}.txt", file_name).as_str())
            {
                return Err(e);
            }
        }
        Ok(())
    }
}

pub mod tablgen {
    use std::{
        fs::File,
        io::{Read, Write},
        path::Path,
        str::FromStr,
    };

    use super::{randi64, tablmgr};

    #[allow(dead_code)]
    pub fn gen_test_table() {
        let gen_table_name =
            String::from_str("testing_table").expect("[ERROR] failed migration to &str");

        let output = Path::new("../static_data/names.txt");

        let mut src = File::open(output).expect("Open file {output}");
        let mut lines = String::new();

        if let Err(e) = src.read_to_string(&mut lines) {
            eprintln!("[ERROR] {}", e);
        }

        let mut gen_table = tablmgr::create(gen_table_name, 'a');
        gen_table.write("stdnt_id,var_id\n".as_bytes()).unwrap();

        for i in 1..lines.replace("\r", "").split('\n').count() - 1 {
            let data = format!("{},{}\n", i, randi64::asm_random(12));
            gen_table
                .write(data.as_bytes())
                .expect("[ERROR] failed to write file!");
        }
    }
}

mod tabltools {
    use std::{fs::File, io::Read};

    pub(super) fn uniq_check(file: &mut File, data: &str) -> bool {
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        for i in buf.split('\n').into_iter() {
            if i.eq(data) {
                return false;
            }
        }
        true
    }
}
