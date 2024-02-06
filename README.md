# pwdgenz
![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/KerbsOpenSource/pwdgenz/total)  ![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/KerbsOpenSource/pwdgenz/release.yml)  ![GitHub last commit](https://img.shields.io/github/last-commit/KerbsOpenSource/pwdgenz)

Command line password generator

## 🤔 What is this?
This is a utility that can generate passwords with a "smart" percentage of character selection. If the first character was a numeric character, when the second character is selected, the program reduces the chance of selecting a numeric character by 2 times, and the other characters increase their selection probability, and so on. 

Standard chances of **!SELECTING A TYPE OF SYMBOL!**
> + Numbers - 32%
> + Lowercase letters - 32%
> + Uppercase letters - 32%
> + Special characters ('!', '@', '#', '$', '-') - 4%

Standard password length - 16 characters

## 👨🏻‍💻 How to use
```
Usage: pwdgenz [OPTIONS]

Options:
  -a, --amount <AMOUNT>  Amount of passwords [default: 1]
  -l, --length <LENGTH>  Password length [default: 16]
  -h, --help             Print help
  -V, --version          Print version
```


## 🔨 How to build
```
git clone https://github.com/KerbsOpenSource/pwdgenz
cd pwdgenz
cargo build -r
```


### License
+ MIT License ([LICENSE file](./LICENSE) or http://opensource.org/licenses/MIT)
