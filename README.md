Currently, this only works under linux.

I used rustup to install nightly-2016-07-04

To build:

```
git clone https://github.com/fxbox/quirc-rs.git
cd quirc-rs
git submodule update --init
cargo run --example rscam_win
```

I used an Android app called
[QR Code Generator](https://play.google.com/store/apps/details?id=com.avlstuff.qrcode)
by AVLStuff.com which allows a variety of QR codes to be generated. You can
show the QR Code on the phone to the camera or download it and print it out
and hold up a printed piece of paper.

