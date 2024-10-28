mod args;
use std::fs::read_to_string;
use clap::Parser;
use args::SlurmArgs;
use std::net::TcpStream;
use ssh2::Session;
use std::io::prelude::*;
use std::io::Read;



/*
 *Author Gaurav Sablok
 *Universitat Potsdam
 *Date: 2024-10-28
 * A rustlang based dotenv reader and file uploader and downloader to
 * high performance computing server or clusters. store your username,
 * ipaddress and password in the dotenv file and then just provide the
 * file with the one line fileto upload and the hostdirectory path.
 *
 *
 * There is no need to execute the ssh command everytime. It establishes
 * a tcp tunnel and send them to the host dir.
 *
 */


fn main() {

    let args:SlurmArgs = SlurmArgs::parse();
    let _  = dotenv_vault::dotenv();
    let ip = std::env::var("IP").unwrap_or("".to_string());
    let user = std::env::var("USER").unwrap_or("".to_string());
    let password = std::env::var("PASSWORD").unwrap_or("".to_string());
    let upload = args.upload_arg.to_string();
    let host_dir = args.hostdir_arg.to_string();
    let mut file_vec = Vec::new();

    for line in read_to_string(&upload).unwrap().lines(){
        let append_line = line;
        file_vec.push(append_line.to_string())
    }

    let mut tcp_file = vec![];

    for i in file_vec.iter() {
       let store_line = format!("scp {} {}@{}:{}", &i,&user, &ip, &host_dir).to_string();
       tcp_file.push(store_line)
    }

    let tcp = TcpStream::connect(ip).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password(&user, &password).unwrap();
    assert!(sess.authenticated());

    let mut channel = sess.channel_session().unwrap();

    for i in tcp_file.iter(){
       let _ = channel.exec(i);
    }

    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close();
    println!("{}", channel.exit_status().unwrap());
}
