An basic emigma machine implemented on rust : https://en.wikipedia.org/wiki/Enigma_machine


encrypts and decrypts messages.

Elements

1- Rotors: have an initial configuration for encryption and decryption
2- Reflectors: return the signal back to the rotors
3- PlugBoard: inverts some configurations

Operation: Enter a word without spaces or periods and you will get that word encrypted with the configuration specified in the rotors.
Enter the same word with the same configuration in the rotors and you will get the message in clear.
