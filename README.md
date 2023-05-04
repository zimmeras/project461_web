Functionality:

"./run install" will run the python script called "localclone_installer.py" in the 'install' directory

    -does this straight from bash script


"./run build" will run the command "cargo build" 

    -this command builds "main.rs" as well as "calculate_ramp_up.rs" at the same time 
    -this command also installs dependencies that are needed for the rust code. these are specified 'Cargo.toml' file


"./run URL_FILE" will run the command "./target/debug/main URL_FILE"

    -this command runs the executable from the "main.rs" rust file
    -will print formatted output that inlcudes all metrics except for BusFactor which we omitted


"./run test" will run the test suite

    -this command runs a test suite consisting of 20 separate test cases
    -10 good and 10 bad urls are included in this test suite
    -number of passed tests and line coverage % are printed to output 


FOLLOW THE SPECIFICATIONS
