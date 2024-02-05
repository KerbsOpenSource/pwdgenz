# pwdgenz
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
### Input - standart
> pwdgenz
> 
Output - String of 16 characters
> W7v9n1M6OhiY3X!0
>

### Input - selectable number of characters
> pwdgenz 32
>
Output - String of 32 characters
> 5bsZDq3qELO5x3PNk8l2@R$LM@hG-k7#

### Input - selectable number of characters + selectable number of passwords
> pwdgenz 32 3
>
Output - 3 Strings of 32 characters each
> q82#JqR0-H3fd-Y7HwE41-6!DX@bfnUP  
> dAH194z#nRh9BeQI7nuuK9!yAQ8-c@j@  
> 2#wU2m-K!@Bn77R$D5R@!!W#05I2Uz-d  


## 🔨 How to build
```
git clone https://github.com/KerbsOpenSource/pwdgenz
cd pwdgenz
cargo build -r
```


### License
+ MIT License ([LICENSE file](./LICENSE) or http://opensource.org/licenses/MIT)