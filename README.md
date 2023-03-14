# bitcoinvert

> A simple currency conversion tool for your CLI

`bitcoinvert` helps you to quickly get conversions between Fiat and Bitcoin right from your CLI,
by leveraging the [blockchain.info ticker API](https://blockchain.info/ticker).

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#installation">Installation</a>
      <ul>
        <li><a href="#linux">Linux</a></li>
      </ul>
    </li>
    <li>
      <a href="#user-manual">User manual</a>
      <ul>
        <li><a href="#basic-example">Basic example</a></li>
        <li><a href="#clean-output-for-piping">Clean output for piping</a></li>
        <li><a href="#no-floating-point">No floating point</a></li>
        <li><a href="#multiple-output-currencies">Multiple output currencies</a></li>
        <li><a href="#other-inputs-missing">Other inputs missing</a></li>
      </ul>
    </li>
    <li><a href="#configuration">Configuration</a></li>
    <li>
      <a href="#configuration">Configuration</a>
      <ul>
        <li><a href="#sample-config">Sample config</a></li>
      </ul>
    </li>
    <li>
      <a href="#supported-currencies">Supported currencies</a>
      <ul>
        <li><a href="#bitcoin">Bitcoin</a></li>
        <li><a href="#fiat">Fiat</a></li>
      </ul>
    </li>
    <li><a href="#contribute">Contribute</a></li>
  </ol>
</details>


## Installation
### Linux
1. Download Linux binary from [Releases](https://github.com/gcomte/bitcoinvert/releases)
2. unzip file: `tar -xvf bitcoinvert[...].tar.gz`
3. move file to executable path: `sudo install -m 0755 -o root -g root -t /usr/local/bin bitcoinvert`

### MacOS
1. Download MacOS binary from [Releases](https://github.com/gcomte/bitcoinvert/releases)
2. unzip file `unzip bitcoinvert[...].zip`
3. move file to executable path: `sudo mv bitcoinvert /usr/local/bin`


If you want, use an alias for `bitcoinvert`, like `bcv`:  
`echo "alias bcv='bitcoinvert'" >> ~/.bash_aliases` for bash
`echo "alias bcv='bitcoinvert'" >> ~/.zshrc` for zsh

#### No build for your architecture?
If you don't find a build for your architecture, compile the binary yourself:    
`cargo build --release`

You can then find the binary here:  
`target/release/bitcoinvert`

Continue with `step 3` from above.

## User manual

`bitcoinvert [OPTIONS] [AMOUNT] [INPUT_CURRENCY] [OUTPUT_CURRENCY]`

### Basic example
`bitcoinvert -c 1 BTC SAT`  
Returns: `100,000,000 SAT`

### Clean output for piping
If you want your result to be lean and ready to be piped into another command, use the `-c` flag:    
`bitcoinvert -c 1 BTC SAT`  
This will remove the commas and the unit and simply return `100000000`.

### No floating point
If you want to get rid of the floating point and display rounded integers instead, use the `-i` flag:  
`bitcoinvert -i 1234567 SAT USD`

### Multiple output currencies
If you don't define the output currency, a table of various currencies will be displayed instead:  
`bitcoinvert -i 1 BTC`

Returns:
```
 unit | amount          
------+-----------------
 BTC  | 1               
 SAT  | 100,000,000     
 MSAT | 100,000,000,000 
 USD  | 22,925          
 EUR  | 21,114          
 GBP  | 18,503
```

### Other inputs missing
If the input currency is missing, `bitcoinvert` will resort to a default instead (e.g. `SAT`, configurable):    
`bitcoinvert 1337`

If the amount is missing, `bitcoinvert` will resort to a default value (e.g. `1 BTC`, configurable):    
`bitcoinvert`

## Help
Run `bitcoinvert --help` to get a concise manual.

## Configuration
The configuration of your defaults is stored in your config folder (`~/.config/bitcoinvert/defaults.yaml` on Linux).  
It defines what values `bitcoinvert` will use if you don't specify them in the command line.

### Sample config
```yaml
amount: 100000000.0
input_currency:
  BitcoinUnit: SAT
output_currencies:
  - BitcoinUnit: BTC
  - BitcoinUnit: SAT
  - BitcoinUnit: MSAT
  - Fiat: USD
  - Fiat: EUR
  - Fiat: GBP
```

## Supported currencies

### Bitcoin
unit | description
--- | ---
`BTC` | bitcoin
`MBTC` | milli-bitcoin
`BITS` | μBTC, micro-bitcoin
`SAT` | satoshi
`MSAT` | milli-satoshi

### Fiat
unit | description
--- | ---
`ARS` | Argentine peso
`AUD` | Australian dollar
`BRL` | Brazilian real
`CAD` | Canadian dollar
`CHF` | Swiss franc
`CLP` | Chilean peso
`CNY` | Chinese yuan (renminbi)
`CZK` | Czech koruna
`DKK` | Danish krone
`EUR` | Euro
`GBP` | Pound sterling
`HKD` | Hong Kong dollar
`HRK` | Croatian kuna
`HUF` | Hungarian forint
`INR` | Indian rupee
`ISK` | Icelandic króna
`JPY` | Japanese yen
`KRW` | South Korean won
`NZD` | New Zealand dollar
`PLN` | Polish złoty
`RON` | Romanian leu
`RUB` | Russian ruble
`SEK` | Swedish krona
`SGD` | Singapore dollar
`THB` | Thai baht
`TRY` | Turkish lira
`TWD` | New Taiwan dollar
`USD` | United States dollar

## Contribute

Contributions are welcome!  
Please craft a PR and direct it towards the `develop` branch.  
The `master` only contains stable versions.
