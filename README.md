# What it does
Scan an IP range using ping and write the used and unused IPs into separate files **inside the directory where the program is run**.
> The files are rewritten on each rerun.

# flags
**-m** or **--main-subnet**:

    part of your ip which is constant.

**-r** or **--ranges-start-stop**

    ranges start and stop to scan them.

**-t** or **--ping-timeout**

    ping timeout in seconds which default is 0.3.

# usage
## using binary
1. download binary from release page (first file)
2. make is executable
    ```bash
    chmod +x available-ips-scanner
    ```
3. run binary with flags
    ```bash
    ./available-ips-scanner -m 192.x.x -r start,stop start,stop
    ```
    or
    ```bash
    ./available-ips-scanner -m 192.x.x -r start,stop -r start,stop
    ```
## using packages
1. download **.deb**/**.rpm** file
2. install packages
    ```bash
    sudo dpkg -i <deb file>
    ```
    or
    ```bash
    sudo rpm -i <rpm file>
    ```
3. use installed binary
    ```bash
    available-ips-scanner -m 192.x.x -r start,stop start,stop
    ```
    or
    ```bash
    available-ips-scanner -m 192.x.x -r start,stop -r start,stop
    ```
## using cargo
1. clone project
2. run using "**cargo run**" in cloned directory
    ```bash
    cargo run -- --main-subnet 192.x.x --ranges-start-stop start,stop start,stop ... <ranges>
    ```

### why i get ip ranges start and ends from cli instead of reading user prompt

using while loop has cost of i/o systemcall to wait for user input and unwrapping and parsing costs

but using cli args we don't have systemcall waiting and input wrapping/parsing cost is less by using clap paser and args type structure, also reading from memory has high performance rather than reading user input. in other hand it may cause annoying cli args writing if have many (~255) range but it's rarely usage.
also it costes spliting input and ad storing new arrays (development complexity cost).