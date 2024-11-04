# Installing the program
To run the program, you need to install 'Rust' and consequently, 'Cargo' which is a package manager.

You can install Rust form this link: https://www.rust-lang.org/tools/install

# Running the program
You can run the program by going to the project directory in the terminal and entering the command 'cargo run --release'. This will compile the program into a binary executable and run it after it is done compiling. To save the terminal output into a text file enter the command 'cargo run --release > <file_name>.txt.'

# Installing additional packages
In the case of an error occurring while compiling the program due to missing packages, you can enter the command 'cargo add <package name>', which will install additional packages to the project directory. For this specific project you should enter 'cargo add rand tabled'.