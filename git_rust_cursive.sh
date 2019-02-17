#bin/bash


printf '\n ---- gas_rust_cursive_1 Start ---- \n'
cd ~/code_projects/rust_projects/rust_projects1/gas_rust_cursive_1
git add -A
# git add *
git commit -m "raz_IT: $(date)"
git push --repo git@github.com:raizul8/gas_rust_cursive_1.git
printf '\n ---- gas_rust_cursive_1: Done ---- \n'