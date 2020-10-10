echo "RESULTS:" > res.txt

export N=10
export SIZE=10
cargo run >> res.txt
echo "\n\n\n" >> res.txt

export N=10
export SIZE=100
cargo run >> res.txt
echo "\n\n\n" >> res.txt

export N=10
export SIZE=1000
cargo run >> res.txt
echo "\n\n\n" >> res.txt

export N=10
export SIZE=10000
cargo run >> res.txt
echo "\n\n\n" >> res.txt

export N=1
export SIZE=100000
cargo run >> res.txt
echo "\n\n\n" >> res.txt

export N=1
export SIZE=1000000
cargo run >> res.txt
echo "\n\n\n" >> res.txt
