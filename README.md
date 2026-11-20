# dkcp

This is a custom file and directory copy program that copies a file or directory to a specified destination. When writing the source and destination, ensure that they exist, otherwise an error will be returned.

## Installation

```bash
git clone https://github.com/Darethan026/dkcp.git && cd dkcp && cargo build --release && sudo cp target/release/dkcp /usr/local/bin
```

## How to use:

```bash
dkcp <source> <destination_dir>
dkcp <source> <destination_dir> --name <desired_file_name.ext>
dkcp <source> <destination_dir> --name <desired_dir_name>
```

## Why dkcp?

- **Uses a 64KB buffer:** Streams file contents without overloading RAM or without rendering the system unresponsive while copying larger files.
- **Displays real-time progress:** Compared to GNU `cp`, `dkcp` gives visual progress on file copying, allowing you to see whether or not your files have copied or are copying.
- **Safe file copying:** By default, `dkcp` does not overwrite existing files and currently does not have a flag to do so.
- **Uses easy to remember arguments:** The same command is used to copy a file or a folder to a directory with no need to use `-r`, but the only difference is when using the `--name` argument, where for files you specify your desired file name and extension, and for folders you just specify the folder name.
- **Written in Rust:** Offers fast copying speeds, but slightly slower than `cp`, although with the added benefits listed above.

## Here's an example of how it'll look when you copy a file to a directory (with time included) - compared with GNU `cp`:

**Specs:**
- Laptop: HP Elitebook 840 G1
- CPU: Intel Core i7-4600U (4th gen)
- RAM: 4GB DDR3L
- Storage: WD Blue (WDC WD5000LPZX-22Z10T0) 500GB HDD @5400 RPM
- OS: Ubuntu 24.04.5 LTS (Kernel: Linux 7.0.0-38-generic)

(`dkcp`)

Run 1 :

```bash
[user]@[device_name]:~/Desktop$ time (dkcp ~/Documents/Win10_22H2_EnglishInternational_x64v1.iso test_folder/ && sync)
Copying: /home/[user]/Documents/Win10_22H2_EnglishInternational_x64v1.iso
Progress: 100.00%	 [5.71 GiB/5.71 GiB]
Successfully copied: /home/[user]/Documents/Win10_22H2_EnglishInternational_x64v1.iso to /home/[user]/Desktop/test_folder

real	2m22.711s
user	0m0.724s
sys	    0m11.221s
```
Run 2:

```bash
[user]@[device_name]:~/Desktop$ time (dkcp ~/Documents/Win10_22H2_EnglishInternational_x64v1.iso test_folder/ && sync)
Copying: /home/[user]/Documents/Win10_22H2_EnglishInternational_x64v1.iso
Progress: 100.00%	 [5.71 GiB/5.71 GiB]
Successfully copied: /home/[user]/Documents/Win10_22H2_EnglishInternational_x64v1.iso to /home/[user]/Desktop/test_folder

real	2m20.524s
user	0m0.728s
sys	    0m11.543s
```

`cp`

Run 1:

```bash
[user]@[device_name]:~/Desktop$ time (cp ~/Documents/Win10_22H2_EnglishInternational_x64v1.iso test_folder/ && sync)

real	2m26.165s
user	0m0.001s
sys	    0m9.888s
```

Run 2:

```bash
[user]@[device_name]:~/Desktop$ time (cp ~/Documents/Win10_22H2_EnglishInternational_x64v1.iso test_folder/ && sync)

real	2m18.561s
user	0m0.003s
sys	    0m10.805s
```
