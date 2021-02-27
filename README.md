This repo is point in time to show learning speed and applicability of non-Rust background to Rust programming, see further progress at https://github.com/kermit4/first_8_hours_of_rust  

If run with no args, it will listen for uploads - not yet working

With args it will send them - unwritten


It's pulled from the server side, once the client sends the first block.

Every 100 packets adds one to the number in flight to increase the in-flight window. 
