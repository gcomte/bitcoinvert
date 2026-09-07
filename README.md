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
        <li><a href="#precision-and-rounding">Precision and rounding</a></li>
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
If you don't find a binary for your architecture, compile it yourself:    
`cargo build --release`

You can then find the binary here:  
`target/release/bitcoinvert`

Continue with `step 3` from above.

## User manual

`bitcoinvert [OPTIONS] [AMOUNT] [INPUT_CURRENCY] [OUTPUT_CURRENCY]`

Invalid amounts and explicit currency codes produce an error and a nonzero exit
status. Defaults are used only for arguments you omit; a misspelled output currency
does not fall back to the default conversion table. Amounts must be finite, and SI
suffixes such as `k`, `M`, and `μ` are supported.

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

### Precision and rounding
Amounts and exchange rates use exact decimal arithmetic. Results round to the nearest
displayed unit, with halfway values rounded away from zero. The `-i` flag applies the
same rule to whole units of the output currency.

Every Bitcoin denomination preserves millisatoshi precision:

Unit | Decimal places
--- | ---
`BTC` | 11
`MBTC` | 8
`BITS` | 5
`SAT` | 3
`MSAT` | 0

For example, `bitcoinvert 1500 MSAT BTC` returns `0.000000015 BTC`, and
`bitcoinvert 100500 MSAT BITS` returns `1.005 BITS`. Amounts smaller than half a
millisatoshi round to zero; exactly half a millisatoshi rounds away from zero.
Fiat currencies keep their existing number of decimal places (usually two).

Decimal input, scientific notation (such as `1.005e-3`), SI suffixes, configured
amounts, and API rates retain their decimal digits through the conversion. Only the
final result is rounded, including conversions between two fiat currencies.
Trailing decimal zeros are omitted from output.

Numbers support up to 309 integer digits and 308 fractional decimal places.
Scientific exponents, including any SI suffix, must be between -308 and 308;
amount input is limited to 1024 bytes. Values outside these bounds, conversion
overflow, and nonpositive or invalid exchange rates produce an error and no result.

### Using SI suffixes for the amount
For very big or small numbers, it's easier to use SI suffixes than adding a lot of zeros.  
`bitcoinvert 1M SAT USD` => convert 1,000,000 SAT to USD  
`bitcoinvert 1.23k SAT` => convert 1,230 SAT

Find a list of possible suffixes [here](https://en.wikipedia.org/wiki/Metric_prefix#List_of_SI_prefixes).

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
The amount may be an unquoted decimal or a quoted decimal string; both preserve
their exact value. Newly generated defaults use a quoted amount.

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
`GHS` | Ghanaian cedi
`HKD` | Hong Kong dollar
`HUF` | Hungarian forint
`INR` | Indian rupee
`ISK` | Icelandic króna
`JPY` | Japanese yen
`KRW` | South Korean won
`NGN` | Nigerian naira
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

Pull Requests are welcome!  
