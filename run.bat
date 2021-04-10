cargo build
cd target/debug
xcopy /E /I "src\textures" "target\debug\textures"
rsdf.exe