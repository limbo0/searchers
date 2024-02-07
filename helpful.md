<!-- copy from the terminal output -->

pbcopy

<!-- remove commas -->
<!-- encoded.txt is the file path to be edited -->

tr -d ',' < encoded.txt | pbcopy

<!-- remove spaces -->

tr -d ' ' < encoded.txt | pbcopy

<!-- compact json file in a single line -->

jq -c '.' input.json | pbcopy

<!-- compact json abi while compiling the contract -->

forge build --silent --via-ir && jq -c '.abi' ./out/TargetSwap.sol/TargetSwap.json | pbcopy
