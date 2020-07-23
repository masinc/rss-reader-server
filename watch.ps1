$ErrorActionPreference = "Stop"

Get-Command systemfd > $null
Get-Command cargo-watch > $null

systemfd --no-pid -s http::3000 -- cargo watch -x run
