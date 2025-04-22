# Aqua Contracts:
- Router: https://stellar.expert/explorer/public/contract/CBQDHNBFBZYE4MKPWBSJOPIYLW4SFSXAXUTSXJN76GNKYVYPCKWC6QUK
- Swap Chained 
- Swap directo tambien lo envia como swap chained

Swap 3 hops variable
https://stellar.expert/explorer/public/tx/aef69e0158e9fe689b123f1b174b2133b34f98da4a29f3b972b6b91fd9791b1c
swap_chained(
    GA3C…T5SO, 
    [
        [
            [CAS3…OWMA, CAUI…OJPK], 
            suAvz8pslvitXL2E53hKd3s22clqJFlALE9FhGKqt/A=bytes, 
            CAUI…OJPK // the next token in the hop
        ], 
        [
            [CAUI…OJPK, CDOF…U2P4], 
            suAvz8pslvitXL2E53hKd3s22clqJFlALE9FhGKqt/A=bytes, 
            CDOF…U2P4 // the next token in the hop
        ], 
        [
            [CCW6…MI75, CDOF…U2P4], 
            bi7nVssQUCDL1liedhAVGYxMhfLoXh5NN/KAtit4gw4=bytes, 
            CCW6…MI75 // the final token to obtain
        ]
        
    ], 
    CAS3…OWMA, 
    100000000u128, 
    25701927u128) → 25961542u128

Swap 1 hop variable
https://stellar.expert/explorer/public/tx/4724ed29ae79bf3c8ca031e10564916a878df8a9e5215980e92d5a467ef4a622

swap_chained(
    GA3C…T5SO, 
    [
        [
            [CAS3…OWMA, CAUI…OJPK], 
            suAvz8pslvitXL2E53hKd3s22clqJFlALE9FhGKqt/A=bytes, 
            CAUI…OJPK // the final token to obtain
        ]
    ], 
    CAS3…OWMA, 
    100000000u128, 
    37785920757u128) → 38167596724u12


Swap 1 hop variable
https://stellar.expert/explorer/public/tx/e066acf7f6eb3d3ecb7f0bca70b1fc64ad529f738352c6af3091325a382738e1
swap_chained(
    GA3C…T5SO, 
    [
        [
            [CCW6…MI75, CDIK…FJKP], 
            yy5OG7xqTdImPNVSp7ZU4S499xjgaGS//WWSMG+HFb4=bytes, 
        CDIK…FJKP]], 
    CCW6…MI75, 
    100000000u128, 
    98918707u128) → 99917886u128

This also works with stable pool with more than 2 tokens:
https://stellar.expert/explorer/public/contract/CD6VHCKSUPGQVQPEQUI6EAEO6Z4PXMFTPHW3UTAOF7W4UF7TH7ZSKZBG
swap_chained(
    GB3J…NDBC,
    [
        [
            [CAS3…OWMA, CCW6…MI75], 
            suAvz8pslvitXL2E53hKd3s22clqJFlALE9FhGKqt/A=bytes, 
            CCW6…MI75
        ], 
        [
            [CCW6…MI75, CDIK…FJKP, CDOF…U2P4], 
            2AETnx8GUW8X4LAUztB8IFsEQUHi14aUE/60BCyHPss=bytes, 
            CDIK…FJKP]
            ], 
        CAS3…OWMA, 
    220000000u128, 
    57425672u128) → 57468384u128