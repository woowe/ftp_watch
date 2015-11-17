#![crate_type = "lib"]
#![crate_name = "ftp_watch"]

// TODO:
// 0.  Use rsnotify, ftp
// 1.  add files to watch
// 2.  remove files to watch
// 3.  quit
// 4.  auto save files based on a predacite
// 5.  watch files on the ftp server for changes so,
//     if the file changes when the user is changing a older version of the file
//     it will notify the client of that
// 6.  take in a config file
// 7.  option to use a custom port to listen on for commands
// 8.  unique directory in the same folder as the binary {ftp-<ip>/}
// 9.  since only pro ftp servers support more advanced file informations, watch the files on the
//     ftp server by comparing the compressed original version (produced after a save or startup of the
//     program) with the version before a save
// 10. be able to use multiple config files
// 11. be able to change root directiory

use std::collections::HashMap;
use std::path::Path;


#[derive(Debug)]
enum FtpWatchErrors {
    FtpConnectErr
}

struct WFile {
    path: Path,
    meta: Option<Metadta>
}

struct WatchFile<T>{
    file: WFile,
    save_pred: Box<Fn(T) -> bool>
}

type FileMap = HashMap<WFile, WatchFile<T>>;

#[derive(Debug)]
struct FtpWatch<T> {
    ftp_server:     String,
    watch_files:    FileMap,
    unique_dir:     Box<Fn(String) -> Path>,
    root_dir:       Path,
    port:           u32,
    conf_file:      Option<Path>,
    username:       String,
    password:       String,
    ftp_stream:     Option<FTPStream>
}

impl FtpWatch<T> {
    pub fn new<T>(server: String, user: String, pass: String, root_dir: Path, port: u32) -> FtpWatch<T> {
        FtpWatch {
            ftp_server:     sever,
            watch_files:    HashMap::new(),
            unique_dir:     Path::new(&format!("ftp-{}", server)),
            root_dir:       root_dir,
            port:           port,
            conf_file:      None,
            username:       user,
            password:       pass,
            ftp_stream:     None
        }
    }

    pub fn set_ftp_server(&mut self, server: String, user: String, pass: String) {
        self.ftp_server = server;
        self.username   = user;
        self.password   = pass;
    }

    pub fn set_watch_files(&mut self, watch_files: FileMap) {
        self.watch_files = watch_files;
    }

    pub fn set_unique_dir(&mut self, unique_dir: Path) {
        self.unique_dir = unique_dir;
    }

    pub fn set_root_dir(&mut self, root_dir: Path) {
        self.root_dir = root_dir;
    }

    pub fn set_port(&mut self, port: u32) {
        self.set_port = port;
    }

    pub fn add_file(&mut self, p: &Path) -> Result<(), FtpWatchErr> {
        
    }

    pub fn set_conf_file(&mut self, conf_file: Path) {
        self.conf_file = Some(conf_file);

        let mut contents = String::new();

        {
            let mut f = File::open(&self.conf_file[..]).unwrap();
            f.read_to_string(&mut contents).unwrap();
        }

        let mut paths: Vec<String> = serde_json::from_str(&contents).unwrap();

        paths.retain(|path| {
        });
    }

    pub fn connect(&mut self) -> Result<(), FtpWatchErr> {
        self.ftp_stream = match FTPStream::connect(self.ftp_server, self.port) {
            Ok(s) => {
                let _ = create_dir(self.root_dir);
                Ok(s)
            },
            Err(e) => return Err(FtpWatchErr::FtpConnectErr)
        }
    }
}

#[test]
fn it_works() {
}
