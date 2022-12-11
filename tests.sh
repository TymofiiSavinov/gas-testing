
contract=dev-1670785272066-17687747370547

# spent gas info from explorer
# 5TGas
near call $contract test1 '{}' --accountId $contract --gas 300000000000000
# 11TGas
near call $contract test2 '{}' --accountId $contract --gas 300000000000000
# 11TGas
near call $contract test3 '{}' --accountId $contract --gas 300000000000000
# 11TGas
near call $contract test4 '{}' --accountId $contract --gas 300000000000000
# 17TGas
near call $contract test5 '{}' --accountId $contract --gas 300000000000000
# 17TGas
near call $contract test6 '{}' --accountId $contract --gas 300000000000000
# 17TGas
near call $contract test7 '{}' --accountId $contract --gas 300000000000000
# 23TGas
near call $contract test8 '{}' --accountId $contract --gas 300000000000000
# 23TGas
near call $contract test9 '{}' --accountId $contract --gas 300000000000000
# 23TGas
near call $contract test10 '{}' --accountId $contract --gas 300000000000000
# 17TGas
near call $contract test11 '{}' --accountId $contract --gas 300000000000000
# 17TGas
near call $contract test12 '{}' --accountId $contract --gas 300000000000000
# 63TGas
near call $contract test13 '{}' --accountId $contract --gas 300000000000000
# 12TGas
near call $contract test14 '{}' --accountId $contract --gas 300000000000000

# real vs explorer
# 5TGas = 5TGas
near call $contract test1 '{}' --accountId $contract --gas 5000000000000
# 11TGas = 11TGas
near call $contract test2 '{}' --accountId $contract --gas 11000000000000
# 16TGas = 11TGas
near call $contract test3 '{}' --accountId $contract --gas 16000000000000
# 11TGas = 11TGas
near call $contract test4 '{}' --accountId $contract --gas 11000000000000
# 17TGas = 17TGas
near call $contract test5 '{}' --accountId $contract --gas 17000000000000
# 31TGas = 17TGas
near call $contract test6 '{}' --accountId $contract --gas 31000000000000
# 17TGas = 17TGas
near call $contract test7 '{}' --accountId $contract --gas 31000000000000
# 23TGas = 23TGas
near call $contract test8 '{}' --accountId $contract --gas 23000000000000
# 27TGas = 23TGas
near call $contract test9 '{}' --accountId $contract --gas 23000000000000
# 23TGas = 23TGas
near call $contract test10 '{}' --accountId $contract --gas 23000000000000
# 111TGas = 17TGas
near call $contract test11 '{}' --accountId $contract --gas 111000000000000
# 17TGas = 17TGas
near call $contract test12 '{}' --accountId $contract --gas 17000000000000
# 63TGas = 63TGas
near call $contract test12 '{}' --accountId $contract --gas 63000000000000
# 12TGas = 12TGas prepaid 7TGas extra gas work
near call $contract test14 '{}' --accountId $contract --gas 7000000000000

