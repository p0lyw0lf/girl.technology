# Installing diesel_cli on Windows

```ps1
$env:RUSTFLAGS="-l static=libcrypto -l static=libecpg -l static=libecpg_compat -l static=libpgcommon -l static=libpgport -l static=libpgtypes -l static=libpq -l static=libssl -l static=lz4 -l static=zlib -L native=D:\\vcpkg\\installed\\x64-windows-static\\lib"
cargo install diesel_cli --no-default-features --features postgres
```
