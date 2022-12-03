# Advent of code solutions

These are my AoC solutions. I write all of them in Rust.

## Workflow
Paste the [session cookie](#get-the-adventofcodecom-cookie) in a file called `secret_session_cookie` in the root of the repo.

Get the input
```
./helper.sh input 22 1
```
Run the code
```
cargo run -p aoc221
```
Submit the answer
```
./helper.sh submit 22 1 1 answer
```

## Get the adventofcode.com cookie
Login on the [website](https://adventofcode.com/). Then you right-click and select "Inspect". Switch to the "Storage" tab and copy the value of the "session" cookie. Keep that value secret, otherwise others can login as you. Paste it in the file called `secret_session_cookie` in the root of the repo. The content of the file has to be in this format
```
Set-Cookie: session=43280943kjl32849wfjw89342mfw7432j; Domain=adventofcode.com
```