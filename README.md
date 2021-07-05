### Ciphers

<img alg="fuzzy" src="wheel.png" width="600"/> 

This is a package in Rust that implements basics encryption algorithms. Details on the the algorithms implemented can be found in this [book](http://inventwithpython.com/cracking/). Here is the complete list of algos and their respective status of implementation.

| Name                   	| status     	|
|------------------------	|------------	|
| Caesar cipher          	|    done    	|
| Transposition Cipher   	|    done   	|
| Affine Cipher          	| to be done 	|
| Simple Substitution    	| to be done 	|
| Vigenère Cipher           |     done  	|
| One-time Pad Cipher    	| to be done 	|
| Generate Prime Numbers 	| to be done 	|
| Public Key Cipher      	| to be done 	|

### Dependencies

num-traits = "0.2.14"

### Example of use:

Here is a chunk of wikipedia text:

#### Input:

*Jeffrey Preston Jorgensen was born in Albuquerque, New Mexico, on January 12, 1964, the son of Jacklyn (née Gise) and Theodore Jørgensen. His biological great-grandfather John Jørgensen, born 8 January 1872 on the small island of Samsø, then in Holbæk County, Zealand, today a municipality in the Central Denmark Region, Denmark, immigrated around 1900 to Chicago, where he had a son, Theodore "Ted" John Jorgensen (b. 1917) with his wife, Ida Minnie Jorgensen, who was also born in Denmark. This son was the father of Ted Jorgensen (b. 1944), Jeff Bezos' biological father.[better source needed] At the time of his birth, his mother was a 17-year-old high school student and his father was 19. After completing high school despite challenging conditions, Jacklyn attended night school while bringing baby Jeff along. After his parents divorced, his mother married Cuban immigrant Miguel "Mike" Bezos in April 1968. Shortly after the wedding, Mike adopted four-year-old Jorgensen, whose surname was then changed to Bezos.*

encrypting using Caesar cipher with key=8:

```rust
mod sub;

use sub::Caesar;
use sub::Translate;
use std::fs;

fn main() {
    let cypher = Caesar {
        key: 8,
    };
    let contents = fs::read_to_string("yourpath/example.txt").unwrap();
    let encrypt_text = cypher.encrypt(&contents);
    let new_file = "yourpath/example.encrypt.txt";
    fs::write(new_file, encrypt_text).unwrap();
}
```

#### Output:

*rmnnzmg xzmabwv rwzomvamv eia jwzv qv itjcycmzycm, vme umfqkw, wv rivcizg 12, 1964, bpm awv wn rikstgv (vém oqam) ivl bpmwlwzm røzomvamv. pqa jqwtwoqkit ozmib-ozivlnibpmz rwpv røzomvamv, jwzv 8 rivcizg 1872 wv bpm auitt qativl wn aiuaø, bpmv qv pwtjæs kwcvbg, hmitivl, bwlig i ucvqkqxitqbg qv bpm kmvbzit lmvuizs zmoqwv, lmvuizs, quuqozibml izwcvl 1900 bw kpqkiow, epmzm pm pil i awv, bpmwlwzm "bml" rwpv rwzomvamv (j. 1917) eqbp pqa eqnm, qli uqvvqm rwzomvamv, epw eia itaw jwzv qv lmvuizs. bpqa awv eia bpm nibpmz wn bml rwzomvamv (j. 1944), rmnn jmhwa' jqwtwoqkit nibpmz.[jmbbmz awczkm vmmlml] ib bpm bqum wn pqa jqzbp, pqa uwbpmz eia i 17-gmiz-wtl pqop akpwwt abclmvb ivl pqa nibpmz eia 19. inbmz kwuxtmbqvo pqop akpwwt lmaxqbm kpittmvoqvo kwvlqbqwva, rikstgv ibbmvlml vqopb akpwwt epqtm jzqvoqvo jijg rmnn itwvo. inbmz pqa xizmvba lqdwzkml, pqa uwbpmz uizzqml kcjiv quuqozivb uqocmt "uqsm" jmhwa qv ixzqt 1968. apwzbtg inbmz bpm emllqvo, uqsm ilwxbml nwcz-gmiz-wtl rwzomvamv, epwam aczvium eia bpmv kpivoml bw jmhwa.*
