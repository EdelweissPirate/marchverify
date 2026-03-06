# marchverify

## 1
### Set up your project etc. Maybe do an ```anchor test``` just to make sure it's ok. *Then*, add dependecies i.e

```
[dependencies]
anchor-lang = { version = "0.31.1", features = ["init-if-needed"] }
anchor-spl = { version = "0.31.1", features = ["token", "metadata", "associated_token"] }
ephemeral-vrf-sdk = {version = "0.1.2", features = ["anchor"]}
solana-security-txt = "1.1.2"
indexmap = "=2.2.6"
solana-program="2.3.0"
constant_time_eq = "=0.3.1"
```

#### *TIP: Notice the last one is new.*

### Then run 

```
cargo update -p constant_time_eq --precise 0.3.1
```

## 2
### Once that is done test `anchor test` still works.

### Create a github repo if needed, think it has to be public.

#### *TIP: Don't forget switch to devnet - ```solana config set --url devnet```*

### Set up Anchor.toml for devnet deployment etc

## 3
### Then just run 

```
solana-verify build
```

### **Then** push to github repo.

### Then run:
```
anchor deploy
```

## 4
### Then you need to run this nightmare command, only changing the relevant parts:

```
solana-verify verify-from-repo -u devnet --program-id EoeCWM7To3utAF8G2T8fREChRLwbc7FqjTtPfDFPtv3h https://github.com/EdelweissPirate/marchverify --commit-hash 36570d80bc7597120a38845df215cef6aac5c9fc --library-name marchverify --mount-path ./
```
* ### The parts to change are:

    - Program ID
    - public github link
    - the commit hash of the commit you just made 
    - the library name of your program *

## 5
### Finally, bask in the temporary grace of victory. 


