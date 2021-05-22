echo " - Build code"
cargo build

echo " - Copy textures"
xcopy /E /I /y "src\textures" "target\debug\textures"

echo " - Run code"
cd target\debug
rsdf.exe
