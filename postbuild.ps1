rm ~/dev/bin/ironhide.exe
rm ~/dev/bin/.data.db
rm ~/dev/bin/.usr.txt

cp target/release/ironhide_cli.exe ~/dev/bin

cd ~/dev/bin

mv ironhide_cli.exe ironhide.exe

cd ~/dev/rust/ironhide_cli