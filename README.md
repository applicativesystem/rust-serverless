# rust-serverless 

- a rustlang cargo to upload multiple files at present on the remote server. 
- reads the ip address, username and the password from the dotenv. 
- needs the path to the file list.
- add a cron job and a automatic restore to any remote server.  

```
╭─gauavsablok@gauravsablok ~/Desktop/rust/rust-serverless ‹main●›
╰─$ ./target/debug/rust-serverless -h
Usage: rust-serverless <UPLOAD_ARG> <HOSTDIR_ARG>

Arguments:
  <UPLOAD_ARG>   please provide the script for the submission to the slurm
  <HOSTDIR_ARG>  please provide the path to the directory on the host

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Gaurav Sablok
