# KeyPlace

 * Human friendly key derivation
 * Server-assisted + persisted
 * Attack resistant
 * Use it in WASM or _wherever_

# Why

So, you use paired-key cryptography on the internets, but you're getting fed up with silly humans writing their private keys on cocktail napkins? SAD!

What if I told you there were a way for YOUR server to help manage these keys, but in a not-totally-crappy way?

It slices, it dices:

* Securely derive private keys with no persistant storage on the user's station!
* The server helps, but cannot actually see the keys. If your server gets pwned, you still win!
* You get to change your keys and passwords independently!

But wait, there's more:

* Store multiple keys using that one password
* Store each key using multiple passwords
* You can implement secure key recovery techniques

KeyPlace is inspired by the Keybase key derivation algorithm https://book.keybase.io/docs/crypto